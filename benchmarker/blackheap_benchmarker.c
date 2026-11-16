#define _GNU_SOURCE
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define CHUNK_BYTES (1024 * 1024)

/* pwrite may write less than asked for, so loop until everything is out */
static void pwrite_all(int fd, const char *buf, size_t len, off_t off) {
  while (len > 0) {
    ssize_t n = pwrite(fd, buf, len, off);
    if (n == -1) {
      perror("pwrite");
      exit(1);
    }
    buf += n;
    len -= (size_t)n;
    off += n;
  }
}

/* Really write out every byte (no sparse file, no fallocate), otherwise reads of holes or unwritten extents
   are served as zeros without touching the device */
static int create(const char *path, unsigned long long bytes) {
  int fd = open(path, O_CREAT | O_WRONLY | O_TRUNC, 0644);
  if (fd == -1) {
    perror("open");
    return 1;
  }

  /* non-zero pseudo-random content, so nothing in the path can shortcut zeros */
  char *buf = malloc(CHUNK_BYTES);
  for (size_t i = 0; i < CHUNK_BYTES; i++) {
    buf[i] = (char)(1 + rand() % 255);
  }

  unsigned long long off = 0;
  while (off < bytes) {
    size_t len = CHUNK_BYTES;
    if (bytes - off < len) {
      len = (size_t)(bytes - off);
    }
    pwrite_all(fd, buf, len, (off_t)off);
    off += len;
  }

  if (fsync(fd) == -1) {
    perror("fsync");
    return 1;
  }
  free(buf);
  close(fd);
  return 0;
}

int main(int argc, char **argv) {
  if (argc == 4 && strcmp(argv[1], "create") == 0) {
    return create(argv[2], strtoull(argv[3], NULL, 10));
  }

  fprintf(stderr, "usage: %s create <file> <bytes>\n", argv[0]);
  return 1;
}
