#include<stdio.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<unistd.h>
#include<string.h>

int main() {
  int fd = open("./test.txt", O_CREAT | O_WRONLY, 0644);
  char str[] = "lorem ipsum";
  printf("to write:%s\n", str);
  write(fd, str, strlen(str));
  close(fd);

  fd = open("./test.txt", O_RDONLY, 0644);
  char buf[51];
  ssize_t res = read(fd, buf, 51);
  buf[res] = '\0';
  printf("res: %lld\n", (long long int)res);
  printf("main: %s\n", buf);
  close(fd);
}
