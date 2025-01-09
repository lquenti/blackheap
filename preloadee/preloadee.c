#define _GNU_SOURCE
#define unlikely(expr) __builtin_expect(!!(expr), 0)

#include<stdbool.h>
#include<stdlib.h>
#include<stdio.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<time.h>
#include<unistd.h>
#include<dlfcn.h>
#include<string.h>
#include<stdarg.h>
#include<threads.h>

#define CSV_HEADER "classification,io_type,bytes,sec\n"

typedef ssize_t (*io_operation_t)(int fd, void *buf, size_t count);

typedef struct state_t {
  int fp;
  ssize_t (*orig_read)(int fd, void *buf, size_t count);
  ssize_t (*orig_write)(int fd, const void *buf, size_t count);
  int (*orig_open)(const char *path, int oflag, ...);
  int (*orig_close)(int fd);
} state_t;

static state_t *current_state = NULL;

/* I tried to use multiple approaches in order to support both multithreading
 * and multiprocessing. The main problems are
 * - Do not sequentialize access too much (through a global mutex)
 * - Do not increase overhead per I/O call too much
 * - Do not expect any other thread to help you
 *   (As they can go away by a process fork)
 *
 * A solution is valid if two threads never write their logs in the same file
 *
 * After thinking and playing around a lot, I found that there are basically
 * two valid approaches:
 *
 * - Every thread writes in their own file; Every I/O request checks whether
 *   the tid stayed the same; if not, open a new file...
 *
 * - Use an (atomically indexed) double buffer, and once the front buffer is
 *   full the back buffer will be flushed out. If the front buffer is full
 *   while the back buffer still flushes, block.
 *
 * While the second one seems faster, it can block quite a lot for write-heavy
 * jobs. Furthermore, whenever the back buffer writeout happens, it probably
 * invalidates a big chunk of the CPU caches, most-likely resulting in weird
 * spikes. I expected it to be quite lock-free, but used up to 3 mutexes in the
 * end.
 *
 * Thus, we use the TID checks instead. If you still want to have a cool non-
 * blocking double buffer, see
 * <https://gist.github.com/lquenti/58a64f93bcfea2bd5790a0e28ccba282>
 */
thread_local pid_t expected_tid;

static void cleanup_state() {
  free(current_state);
}

static void open_based_on_tid_and_write_header() {
  int timestamp = (int)time(NULL);
  char filename[256];
  sprintf(filename, "./io_recordings_%d_%d.csv", expected_tid, timestamp);
  current_state->fp = current_state->orig_open(filename, O_CREAT | O_WRONLY | O_TRUNC, 0644);
  current_state->orig_write(current_state->fp, CSV_HEADER, strlen(CSV_HEADER));
}


static void init_state() {
  atexit(cleanup_state);
  current_state = malloc(sizeof(state_t));

  expected_tid = gettid();
  current_state->orig_read = dlsym(RTLD_NEXT, "read");
  current_state->orig_write = dlsym(RTLD_NEXT, "write");
  current_state->orig_open = dlsym(RTLD_NEXT, "open");
  current_state->orig_close = dlsym(RTLD_NEXT, "close");

  open_based_on_tid_and_write_header();
}


static inline double timespec_to_double(const struct timespec *time) {
  return time->tv_sec + 0.001 * 0.001 * 0.001 * time->tv_nsec;
}

static double get_duration(const struct timespec *start, const struct timespec *end) {
  return timespec_to_double(end) - timespec_to_double(start);
}

static ssize_t do_io(bool is_read, int fd, void *buf, size_t count) {
  // init state if first time
  if (unlikely(current_state == NULL)) {
    init_state();
  }

  // move branching out of benchmark
  io_operation_t io_op;
  if (is_read) {
    io_op = current_state->orig_read;
  } else {
    io_op = (io_operation_t) current_state->orig_write;
  }

  // do benchmark
  ssize_t res;
  struct timespec start, end;
  double duration;
  clock_gettime(CLOCK_MONOTONIC, &start);
  res = io_op(fd, buf, count);
  clock_gettime(CLOCK_MONOTONIC, &end);
  duration = get_duration(&start, &end);


  // Check if we forked in-between
  // (avoid race condition)
  pid_t current_tid = gettid();
  if (current_tid != expected_tid) {
    expected_tid = current_tid;
    open_based_on_tid_and_write_header();
  }

  // record results
  // (Don't record our recording)
  if (fd != current_state->fp) {
    char result_buf[256];
    sprintf(result_buf,
        "NotYetClassified,\%c,%zu,%.17g\n",
        is_read ? 'r' : 'w',
        res,
        duration
    );
    current_state->orig_write(current_state->fp, result_buf, strlen(result_buf));
  }

  // return actual result
  return res;
}

ssize_t read(int fd, void *buf, size_t count) {
  return do_io(true, fd, buf, count);
}

ssize_t write(int fd, const void *buf, size_t count) {
  return do_io(false, fd, (void *)buf, count);
}

// See: https://elixir.bootlin.com/glibc/latest/source/io/bits/fcntl2.h#L41
// But we know that we either have 2 or 3 arguments.
// Thus we don't have to do the Vararg magic described in
// https://gcc.gnu.org/onlinedocs/gcc-4.7.2/gcc/Constructing-Calls.html
int open(const char *path, int oflag, ...) {
  if (unlikely(current_state == NULL)) {
    init_state();
  }
  va_list args;
  int mflag;

  int ret;
  // only with O_CREAT the third argument is used
  if (oflag & O_CREAT) {
    va_start(args, oflag);
    mflag = va_arg(args, int);
    ret = current_state->orig_open(path, oflag, mflag);
  } else {
    ret = current_state->orig_open(path, oflag);
  }
  return ret;
}

int close(int fd) {
  if (unlikely(current_state == NULL)) {
    init_state();
  }
  return current_state->orig_close(fd);
}
