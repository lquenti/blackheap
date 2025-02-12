#define _GNU_SOURCE
#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>
#include<fcntl.h>
#include<pthread.h>
#include<sys/syscall.h>

#define N_THREADS 10

pthread_barrier_t barrier;

void *thread_func(void *arg) {
  (void)arg; // unused
  // make sure that all threads are started before the first I/O op
  pthread_barrier_wait(&barrier);

  // open based on therad
  pid_t tid = gettid();
  char filename[256];
  sprintf(filename, "./text_%d.txt", tid);
  int fd = open(filename, O_CREAT | O_TRUNC | O_WRONLY, 0644);
  if (fd == -1) {
    perror("Error opening file for writing");
    pthread_exit(NULL);
  }

  // write test
  for (int i=0; i<10; ++i) {
    if (write(fd, "lol\n", sizeof("lol\n")-1) < 0) {
      perror("write failed");
      close(fd);
      pthread_exit(NULL);
    }
  }

  // close
  if (close(fd) < 0) {
    perror("close");
  }

  pthread_exit(NULL);
}

int main() {
  pthread_t threads[N_THREADS];

  if (pthread_barrier_init(&barrier, NULL, N_THREADS) != 0) {
    perror("pthread_barrier_init failed");
    exit(EXIT_FAILURE);
  }

  for (int i=0; i<N_THREADS; ++i) {
    if (pthread_create(&(threads[i]), NULL, thread_func, NULL) != 0) {
      perror("pthread create");
      exit(EXIT_FAILURE);
    }
  }

  for (int i=0; i<N_THREADS; ++i) {
    if (pthread_join(threads[i], NULL) != 0) {
      perror("pthread_join");
      exit(EXIT_FAILURE);
    }
  }

  if (pthread_barrier_destroy(&barrier) != 0) {
    perror("pthread_barrier_destroy");
    exit(EXIT_FAILURE);
  }

}
