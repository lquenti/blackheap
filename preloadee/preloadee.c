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

typedef struct state_t {
  int fp;
  ssize_t (*orig_write)(int fd, const void *buf, size_t count);
} state_t;

static state_t *current_state = NULL;

static void init_state() {
  current_state = malloc(sizeof(state_t));

  int timestamp = (int)time(NULL);
  char filename[256];
  sprintf(filename, "./io_recordings_%d.csv", timestamp);
  current_state->fp = open(filename, O_CREAT | O_WRONLY | O_TRUNC, 0644);

  current_state->orig_write = dlsym(RTLD_NEXT, "write");
}

ssize_t write(int fd, const void *buf, size_t count) {
  if (unlikely(current_state == NULL)) {
    init_state();
  }
  printf("%s\n", (char *)buf);
  char debug_str[] = "Hello World\n";
  current_state->orig_write(current_state->fp, debug_str, strlen(debug_str));

  return current_state->orig_write(fd, buf, count);
}
