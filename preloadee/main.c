#include<stdio.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<unistd.h>
#include<string.h>

int main() {
  int fd = open("./test.txt", O_CREAT | O_WRONLY, 0644);
  printf("Before:\n");
  char str[] = "lorem ipsum";
  write(fd, str, 11);
  close(fd);
}
