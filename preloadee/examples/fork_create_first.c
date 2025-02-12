#define _GNU_SOURCE
#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>
#include<fcntl.h>
#include<sys/syscall.h>

#define N_PROCESSES 10

pthread_barrier_t barrier;

void *child_process(int idx) {
  // make sure that all threads are started before the first I/O op
  sleep(10);

  // open based on therad
  char filename[256];
  sprintf(filename, "./text_%d.txt", idx);
  int fd = open(filename, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd == -1) {
    perror("Error opening file for writing");
    exit(NULL);
  }

  // write test
  for (int i=0; i<10; ++i) {
    if (write(fd, "lol\n", sizeof("lol\n")-1) < 0) {
      perror("write failed");
      close(fd);
      exit(NULL);
    }
  }

  // close
  if (close(fd) < 0) {
    perror("close");
    exit(EXIT_FAILURE);
  }

  exit(EXIT_SUCCESS);
}

int main() {
  pid_t pids[N_PROCESSES];
  for (int i=0; i<N_PROCESSES; ++i) {
    pids[i] = fork();
  }
}

