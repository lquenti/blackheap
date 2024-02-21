#ifndef BLACKHEAP_BENCHMARKER_BENCHMARER_INTERNAL_H
#define BLACKHEAP_BENCHMARKER_BENCHMARER_INTERNAL_H

#include<stdlib.h>
#include"./benchmarker.h"

struct benchmark_state {
  void *buffer;
  int fd;
  size_t last_mem_offset;
  size_t last_file_offset;
  ssize_t (*io_op)(int fd, void *buf, size_t count);
};

#endif
