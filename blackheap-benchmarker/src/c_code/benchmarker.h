#ifndef BLACKHEAP_BENCHMARKER_BENCHMARER_H
#define BLACKHEAP_BENCHMARKER_BENCHMARER_H


#define MEMINFO "/proc/meminfo"

/* https://www.kernel.org/doc/Documentation/sysctl/vm.txt */
#define DROP_PAGE_CACHE "/proc/sys/vm/drop_caches"

#include<stdbool.h>
#include<stddef.h>

/* All possible access patterns */
enum access_pattern {
  ACCESS_PATTERN_CONST = 0,
  ACCESS_PATTERN_SEQUENTIAL = 1,
  ACCESS_PATTERN_RANDOM = 2,
};

enum error_codes {
  ERROR_CODES_SUCCESS = 0,

  /* Linux operations that failed */
  ERROR_CODES_MALLOC_FAILED = 1,
  ERROR_CODES_OPEN_FAILED = 2,
  ERROR_CODES_READ_FAILED = 3,
  ERROR_CODES_WRITE_FAILED = 4,
  ERROR_CODES_LSEEK_FAILED = 5,
  ERROR_CODES_FSYNC_FAILED = 6,
  ERROR_CODES_FSTAT_FAILED = 7,
  ERROR_CODES_IO_OP_FAILED = 8,
  ERROR_CODES_REMOVE_FAILED = 9,

  /* Higher level operations */
  ERROR_CODES_DROP_PAGE_CACHE_FAILED_NO_PERMISSIONS = 10,
  ERROR_CODES_DROP_PAGE_CACHE_FAILED_OTHER = 11,

  ERROR_CODES_INCORRECT_FILE_BUFFER_SIZE = 12,
};


struct benchmark_config {
  const char *filepath;
  const size_t memory_buffer_in_bytes;
  const size_t file_size_in_bytes;
  const size_t access_size_in_bytes;
  const size_t number_of_io_op_tests;
  const enum access_pattern access_pattern_in_memory;
  const enum access_pattern access_pattern_in_file;
  const bool is_read_operation;
  /* Whether the file should be bloated up to file_size_in_bytes.
   *
   * In most cases, this should be true.
   * The only expections are special "files" that can't be made bigger like
   * special devices.
   */
  const bool prepare_file_size;

  /* Note that this requires root */
  const bool drop_cache_first;
  const bool do_reread;
  const size_t restrict_free_ram_to;
};

struct benchmark_results {
  enum error_codes res;
  size_t length;
  double *durations;
};

struct benchmark_results benchmark_file(const struct benchmark_config *config);

#endif
