#define _GNU_SOURCE // Required for gettid()
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <pthread.h>
#include <sys/syscall.h>

#define NUM_THREADS 10
#define FILE_NAME_PREFIX "./test_"
#define FILE_NAME_SUFFIX ".txt"
#define MESSAGE "lol\n"
#define MESSAGE_COUNT 10

pthread_barrier_t barrier;

void* thread_function(void* arg) {
    // Wait for all threads to reach the barrier
    pthread_barrier_wait(&barrier);

    // Get the thread ID
    pid_t tid = syscall(SYS_gettid);

    // Construct the file name
    char file_name[256];
    snprintf(file_name, sizeof(file_name), "%s%d%s", FILE_NAME_PREFIX, tid, FILE_NAME_SUFFIX);

    // Open the file with O_CREAT | O_TRUNC
    int fd = open(file_name, O_CREAT | O_TRUNC | O_WRONLY, 0644);
    if (fd < 0) {
        perror("open");
        pthread_exit(NULL);
    }

    // Write "lol\n" 10 times
    for (int i = 0; i < MESSAGE_COUNT; i++) {
        if (write(fd, MESSAGE, sizeof(MESSAGE) - 1) < 0) {
            perror("write");
            close(fd);
            pthread_exit(NULL);
        }
    }

    // Close the file
    if (close(fd) < 0) {
        perror("close");
        pthread_exit(NULL);
    }

    pthread_exit(NULL);
}

int main() {
    pthread_t threads[NUM_THREADS];

    // Initialize the barrier for NUM_THREADS threads
    if (pthread_barrier_init(&barrier, NULL, NUM_THREADS) != 0) {
        perror("pthread_barrier_init");
        exit(EXIT_FAILURE);
    }

    // Create threads
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, thread_function, NULL) != 0) {
            perror("pthread_create");
            exit(EXIT_FAILURE);
        }
    }

    // Wait for all threads to finish
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("pthread_join");
            exit(EXIT_FAILURE);
        }
    }

    // Destroy the barrier
    if (pthread_barrier_destroy(&barrier) != 0) {
        perror("pthread_barrier_destroy");
        exit(EXIT_FAILURE);
    }

    return 0;
}
