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


#define CSV_HEADER "filename,io_type,bytes,sec\n"

typedef ssize_t (*io_operation_t)(int fd, void *buf, size_t count);

typedef struct fd_table_t {
  size_t n;
  int *fds;
  char **filenames;
} fd_table_t;

typedef struct state_t {
  int fp;
  ssize_t (*orig_read)(int fd, void *buf, size_t count);
  ssize_t (*orig_write)(int fd, const void *buf, size_t count);
  int (*orig_open)(const char *path, int oflag, ...);
  int (*orig_close)(int fd);
  fd_table_t fd_table;
} state_t;

static state_t *current_state = NULL;

static void cleanup_state() {
  // current_state is never a nullptr since this just gets
  // called if init_state() got called first
  free(current_state->fd_table.fds);
  free(current_state->fd_table.filenames);
  free(current_state);
}

static ssize_t get_filename_index_from_fd(int fd) {
  for (size_t i=0; i<current_state->fd_table.n; ++i) {
    if (current_state->fd_table.fds[i] == fd) {
      return i;
    }
  }
  fprintf(stderr, "ERROR: COULD NOT FIND fd: %d\n",fd);
  // TODO better error handling if even possible
  return -1;
}

static void add_to_lookup_table(int fd, const char *str) {
  // expected to be called after init_state

  // copy
  size_t len = strlen(str);
  char *str_heap = malloc(sizeof(char) * len);
  strcpy(str_heap, str);

  // add to table
  current_state->fd_table.n += 1;
  realloc(current_state->fd_table.fds, sizeof(int) * current_state->fd_table.n);
  realloc(current_state->fd_table.filenames, sizeof(int) * current_state->fd_table.n);
  current_state->fd_table.fds[current_state->fd_table.n - 1] = fd;
  current_state->fd_table.filenames[current_state->fd_table.n - 1] = str_heap;
}

static void remove_from_lookup_table(int fd) {
  // expected to be called after init_state

  // get the element in Table (in O(n) lol)
  ssize_t fd_index = get_filename_index_from_fd(fd);
  if (fd_index == -1) {
    return;
  }

  // remove the filename not used anymore
  free(current_state->fd_table.filenames[fd_index]);

  // fill the gap resulted by deletion
  for (size_t i=fd_index; i<(current_state->fd_table.n -1); ++i) {
    current_state->fd_table.fds[i] = current_state->fd_table.fds[i+1];
    current_state->fd_table.filenames[i] = current_state->fd_table.filenames[i+1];
  }

  // reduce size
  current_state->fd_table.n -= 1;
  realloc(current_state->fd_table.fds, sizeof(int) * current_state->fd_table.n);
  realloc(current_state->fd_table.filenames, sizeof(int) * current_state->fd_table.n);
}


static void init_state() {
  atexit(cleanup_state);
  current_state = malloc(sizeof(state_t));
  current_state->fd_table.n = 1;
  current_state->fd_table.filenames = malloc(sizeof(char *));
  current_state->fd_table.fds = malloc(sizeof(int));

  int timestamp = (int)time(NULL);
  char filename[256];
  sprintf(filename, "./io_recordings_%d.csv", timestamp);
  current_state->orig_read = dlsym(RTLD_NEXT, "read");
  current_state->orig_write = dlsym(RTLD_NEXT, "write");
  current_state->orig_open = dlsym(RTLD_NEXT, "open");
  current_state->orig_close = dlsym(RTLD_NEXT, "close");

  current_state->fp = current_state->orig_open(filename, O_CREAT | O_WRONLY | O_TRUNC, 0644);
  current_state->fd_table.fds[0] = current_state->fp;
  current_state->fd_table.filenames[0] = malloc(sizeof(char) * strlen(filename));
  strcpy(current_state->fd_table.filenames[0],  filename);


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
  // (Don't record our recording)
  if (fd != current_state->fp) {
    ssize_t fd_index = get_filename_index_from_fd(fd);
    if (fd_index == -1) {
      // TODO error handling
      return res;
    }

    char result_buf[256];
    sprintf(result_buf,
        "\"%s\",%c,%zu,%.17g\n",
        current_state->fd_table.filenames[fd_index],
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

  va_start(args, oflag);
  mflag = va_arg(args, int);
  int ret = current_state->orig_open(path, oflag, mflag);
  add_to_lookup_table(ret, path);
  return ret;
}

int close(int fd) {
  if (unlikely(current_state == NULL)) {
    init_state();
  }
  remove_from_lookup_table(fd);
  return current_state->orig_close(fd);
}
