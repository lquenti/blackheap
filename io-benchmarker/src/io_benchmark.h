#ifndef IO_BENCHMARK_IO_BENCHMARK_H
#define IO_BENCHMARK_IO_BENCHMARK_H

#include<stddef.h>
#include<stdbool.h>
#include<unistd.h>

#include "helper.h"

/** All possible access patterns. */
typedef enum access_pattern_t
{
  ACCESS_PATTERN_CONST = 0,      /**< always read at the same place */
  ACCESS_PATTERN_SEQUENTIAL = 1, /**< read sequentially through the file (like an intuitive read) */
  ACCESS_PATTERN_RANDOM = 2,     /**< go to a random position after every read */
} access_pattern_t;

/** All possible configuration options for a benchmark run
 * This will get parsed from the command line options.
 */
typedef struct benchmark_config_t
{
  /**< The path to the file from which will be read from or written to */
  const char *filepath;

  /**< The size of the memory buffer to/from which the information will be written/read to*/
  const size_t memory_buffer_in_bytes;

  /**< The size of the file specified by filepath. Ignored is prepare_file_size is set to false. */
  const size_t file_size_in_bytes;

  /**< The size of each I/O request. */
  const size_t access_size_in_bytes;

  /**< The number of tests done for this execution. */
  const size_t number_of_io_op_tests;

  /**< Which access pattern should be used aka how the memory pointer should be moved.*/
  const access_pattern_t access_pattern_in_memory;

  /**< Which access pattern should be used aka how the file pointer should be seeked.*/
  const access_pattern_t access_pattern_in_file;

  /**< Whether the benchmaked I/O-operation is read or write. */
  const bool is_read_operation;

  /** Whether the file should be bloated up to file_size_in_bytes.
   *
   * In most cases, this should be true.
   * The only expections are special "files" that can't be made bigger like
   * special devices.
   */
  const bool prepare_file_size;

  const bool use_o_direct;

  const bool drop_cache_first;

  const bool do_reread;

  const bool delete_afterwards;

  const size_t restrict_free_ram_to;
} benchmark_config_t;

typedef ssize_t (*io_op_t)(int fd, void *buf, size_t count);

/** The current state of the program, wrapped into a struct to declutter global state.
 *
 * In order to not pollute global state we keep our state local to the functions.
 * We wrap it all into a struct to not have functions with 10+ parameters.
 *
 * The state is basically a singleton for each thread, created by the benchmark itself.
 */
typedef struct benchmark_state_t
{
  /**< The memory buffer on which the io-ops read/write from/to */
  void *buffer;

  /**< The file descriptor of the benchmarked file specified by the config struct. */
  int fd;

  /**< The memory offset after the last io-operation, needed to specify the next one according to access pattern. */
  size_t last_mem_offset;

  /**< The file offset after the last io-operation, needed to specify the next one according to access pattern. */
  size_t last_file_offset;
  io_op_t io_op;
} benchmark_state_t;

/** The results returned after the benchmark.
 *
 * Each test, defined by its starting and end time, is for a single io-operation (i.e. a single read or write).
 * No statistical accumulation or means.
 */
typedef struct benchmark_results_t
{
  /**< The number of io-ops measured in this benchmark */
  size_t length;

  /**< An array of durations. The i-th double corresponds to the starting time of the i-th io-operation */
  double *durations;

} benchmark_results_t;

typedef struct timespec timespec_t;
benchmark_results_t *benchmark_file(const benchmark_config_t *config);

#endif //IO_BENCHMARK_IO_BENCHMARK_H
