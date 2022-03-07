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


#define CSV_HEADER "io_type,bytes,sec\n"

typedef ssize_t (*io_operation_t)(int fd, void *buf, size_t count);

typedef struct state_t {
  int fp;
  ssize_t (*orig_read)(int fd, void *buf, size_t count);
  ssize_t (*orig_write)(int fd, const void *buf, size_t count);
  int (*orig_open)(const char *path, int oflag, ...);
  int (*orig_close)(int fd);
} state_t;

static state_t *current_state = NULL;

static void cleanup_state() {
  // current_state is never a nullptr since this just gets
  // called if init_state() got called first
  close(current_state->fp);
  free(current_state);
}

static void init_state() {
  atexit(cleanup_state);
  current_state = malloc(sizeof(state_t));

  int timestamp = (int)time(NULL);
  char filename[256];
  char *msg;
  sprintf(filename, "./io_recordings_%d.csv", timestamp);
  current_state->orig_read = dlsym(RTLD_NEXT, "read");
  current_state->orig_write = dlsym(RTLD_NEXT, "write");
  current_state->orig_open = dlsym(RTLD_NEXT, "open");
  if ((msg = dlerror())) {
    fprintf(stderr, "dlsym error:%s\n", msg);
    exit(1);
  }
  current_state->orig_close = dlsym(RTLD_NEXT, "close");

  current_state->fp = current_state->orig_open(filename, O_CREAT | O_WRONLY | O_TRUNC, 0644);


  // write CSV header
  current_state->orig_write(current_state->fp, CSV_HEADER, strlen(CSV_HEADER));
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

  // record results
  char result_buf[128];
  sprintf(result_buf, "%c,%zu,%.17g\n", is_read ? 'r' : 'w', res, duration);
  printf("%s", result_buf); // TODO remove me
  current_state->orig_write(current_state->fp, result_buf, strlen(result_buf));

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
  char *msg;
  va_list args;
  int mflag;

  va_start(args, oflag);
  mflag = va_arg(args, int);
  int ret = current_state->orig_open(path, oflag, mflag);
  printf("open: %s -> %d\n", path, ret);
  return ret;
}

int close(int fd) {
  if (unlikely(current_state == NULL)) {
    init_state();
  }
  printf("%d closed.\n", fd);
  return current_state->orig_close(fd);
}
