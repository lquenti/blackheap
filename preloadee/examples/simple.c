#include<stdio.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<unistd.h>
#include<string.h>

/* Simplest possible example, with a lot of checking on whether our IO fails */

int main() {
  int fd = open("./test.txt", O_CREAT | O_WRONLY | O_TRUNC, 0644);
  if (fd == -1) {
    perror("Error opening file for writing");
    return 1;
  }

  // write
  char str[] = "lorem ipsum";
  printf("to write: %s\n", str);
  ssize_t bytes_written = write(fd, str, strlen(str));
  if (bytes_written == -1) {
    perror("Error writing to file");
    close(fd);
    return 1;
  }
  if ((size_t)bytes_written != strlen(str)) {
    fprintf(stderr, "Partial write detected. Expected: %zu, Written: %zd\n",
        strlen(str), bytes_written);
    close(fd);
    return 1;
  }

  if (close(fd) == -1) {
    perror("Error closing file after writing");
    return 1;
  }

  fd = open("./test.txt", O_RDONLY);
  if (fd == -1) {
    perror("Error opening file for reading");
    return 1;
  }

  // read
  char buf[51];
  ssize_t bytes_read = read(fd, buf, sizeof(buf) - 1);
  if (bytes_read == -1) {
    perror("Error reading from file");
    close(fd);
    return 1;
  }
  buf[bytes_read] = '\0';

  printf("res: %lld\n", (long long int)bytes_read);
  printf("main: %s\n", buf);

  if (close(fd) == -1) {
    perror("Error closing file after reading");
    return 1;
  }

  return 0;
}

