//
// Created by lquenti on 22.11.21.
//


#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <time.h>

#include"io_benchmark.h"

// Notes:
// Which clocks to use:
//   - https://stackoverflow.com/a/12480485/9958281



/** Initializes the file used for reading/writing.
 *
 * Since write needs a buffer argument, we use our benchmark buffer to fill it, thus
 * we possibly need multiple iterations to fill it.
 * The size is defined by config->file_size_in_bytes.
 */
static void init_file(const benchmark_config_t *config, benchmark_state_t *state)
{
  /* is it externally managed? */
  if (!config->prepare_file_size)
    return;

  state->fd = open_or_die(config->filepath, O_CREAT | O_RDWR, 0644);

  /* Does it already have the correct size? */
  struct stat st;
  fstat_or_die(state->fd, &st);
  close_or_die(state->fd);
  if ((size_t)st.st_size == config->file_size_in_bytes)
    return;

  /* If not, we just truncate it to zero and fill it up */
  state->fd = open_or_die(config->filepath, O_RDWR | O_TRUNC, 0644);
  size_t count = (MAX_IO_SIZE <= config->memory_buffer_in_bytes) ? MAX_IO_SIZE : config->memory_buffer_in_bytes;
  size_t iterations = config->file_size_in_bytes / count;
  for (; iterations; --iterations)
  {
    write_or_die(state->fd, state->buffer, count);
  }
  /* Now allocate the rest which is less than 1 buffer size. */
  size_t rest = config->file_size_in_bytes % count;
  write_or_die(state->fd, state->buffer, rest);

  /* Did it work? */
  fsync_or_die(state->fd);
  fstat_or_die(state->fd, &st);
  if ((size_t)st.st_size != config->file_size_in_bytes)
  {
    fprintf(stderr, "ERROR: File size does not match. Expected: %zu Actual: %zu\n", config->file_size_in_bytes,
            (size_t)st.st_size);
    exit(1);
  }
}

/** Initializes the benchmark_state_t struct with proper values from the config. */
static void init_state(const benchmark_config_t *config, benchmark_state_t *state)
{
  state->buffer = malloc_or_die(config->memory_buffer_in_bytes);
  memset(state->buffer, '1', config->memory_buffer_in_bytes);
  state->last_mem_offset = 0;
  state->last_file_offset = 0;
  state->io_op = config->is_read_operation ? read : (io_op_t)write;
}

/** Sane initialization of the benchmark_results_t struct based on config values. */
static void init_results(const benchmark_config_t *config, benchmark_results_t *results)
{
  results->length = config->number_of_io_op_tests;
  results->durations = malloc_or_die(sizeof(double) * config->number_of_io_op_tests);
}

/** Initialzes the memory pointer for the working memory buffer.
 *
 * Depending on the access patterns.
 */
static inline void init_memory_position(const benchmark_config_t *config, size_t *res)
{
  switch (config->access_pattern_in_memory)
  {
  case ACCESS_PATTERN_CONST:
  case ACCESS_PATTERN_SEQUENTIAL:
  {
    *res = 0;
    return;
  }
  case ACCESS_PATTERN_RANDOM:
  {
    /* TODO: Possible Alignment and Rewrite */
    *res = ((size_t)rand() * 128) % (config->memory_buffer_in_bytes - config->access_size_in_bytes);
    return;
  }
  }
}

/** Initializes the file pointer location.
 *
 * Depending on the access patterns.
 * In our program state we also track the current state
 */
static inline void init_file_position(const benchmark_config_t *config, benchmark_state_t *state)
{
  switch (config->access_pattern_in_file)
  {
  case ACCESS_PATTERN_CONST:
  case ACCESS_PATTERN_SEQUENTIAL:
  {
    // We start at the beginning
    state->last_file_offset = 0;
    return;
  }
  case ACCESS_PATTERN_RANDOM:
  {
    // TODO: Possible alignment and rewrite
    size_t random_offset = ((off_t)rand() * 128) % (config->file_size_in_bytes - config->access_size_in_bytes);
    lseek_or_die(state->fd, random_offset, SEEK_CUR);
    state->last_file_offset = random_offset;
    return;
  }
  }
}

/** Reset all state values before the run. */
static void prepare_run(const benchmark_config_t *config, benchmark_state_t *state)
{
  if (config->use_o_direct)
    state->fd = open_or_die(config->filepath, O_RDWR | O_DIRECT, 0644);
  else
    state->fd = open_or_die(config->filepath, O_RDWR, 0644);
  if (config->restrict_free_ram_to != 0)
    allocate_memory_until(config->restrict_free_ram_to/1024);
  init_memory_position(config, &state->last_mem_offset);
  init_file_position(config, state);
}

/** Choose the next memory position after each io-op according to the access pattern. */
static inline void pick_next_mem_position(const benchmark_config_t *config, benchmark_state_t *state)
{
  switch (config->access_pattern_in_memory)
  {
  case ACCESS_PATTERN_CONST:
    /* After one io-op the pointer does not get moved like the fd-state for the file */
    return;
  case ACCESS_PATTERN_SEQUENTIAL:
  {
    state->last_mem_offset += config->access_size_in_bytes;
    return;
  }
  case ACCESS_PATTERN_RANDOM:
    state->last_mem_offset = ((size_t)rand() * 128) % (config->memory_buffer_in_bytes - config->access_size_in_bytes);
    return;
  }
}

/** Choose the next file position after each io-op according to the access pattern */
static inline void pick_next_file_position(const benchmark_config_t *config, benchmark_state_t *state)
{
  switch (config->access_pattern_in_file)
  {
  case ACCESS_PATTERN_CONST:
    lseek_or_die(state->fd, 0, SEEK_SET);
    return;
  case ACCESS_PATTERN_SEQUENTIAL:
  {
    state->last_file_offset = state->last_file_offset + config->access_size_in_bytes;
    /* we don't have to lseek since the pointer moves naturally */
    return;
  }
  case ACCESS_PATTERN_RANDOM:
  {
    // TODO: Refactor align....
    size_t new_file_pos = ((size_t)rand() * 128) % (config->file_size_in_bytes - config->access_size_in_bytes);
    lseek_or_die(state->fd, new_file_pos, SEEK_SET);
    state->last_file_offset = new_file_pos;
    return;
  }
  }
}

/** Extracts the number of seconds from a struct timespec defined by time.h */
static inline double timespec_to_double(const timespec_t *time)
{
  return time->tv_sec + 0.001 * 0.001 * 0.001 * time->tv_nsec;
}

/** Update the tracked values after each io-operation */
static double get_duration(const timespec_t *start, const timespec_t *end)
{
  return timespec_to_double(end) - timespec_to_double(start);
}

static void do_reread_if_needed(const benchmark_config_t *config, benchmark_state_t *state) {
  if (!config->do_reread)
    return;
  state->io_op(state->fd, state->buffer, config->access_size_in_bytes);
  /* Seek back so that we read it twice */
  lseek_or_die(state->fd, state->last_file_offset, SEEK_SET);
}

/** The actual benchmark function.
 *
 * After preparing, it gets the time before, does the io-op and then gets the time afterwards and updates it.
 */
static void do_benchmark(const benchmark_config_t *config, benchmark_state_t *state, benchmark_results_t *results)
{
  timespec_t start, end;
  int res;
  prepare_run(config, state);
  for (size_t i = 0; i < config->number_of_io_op_tests; ++i)
  {
    do_reread_if_needed(config, state);
    clock_gettime(CLOCK_MONOTONIC, &start);
    res = state->io_op(state->fd, state->buffer, config->access_size_in_bytes);
    clock_gettime(CLOCK_MONOTONIC, &end);
    io_op_worked_or_die(res, config->is_read_operation);
    pick_next_mem_position(config, state);
    pick_next_file_position(config, state);
    results->durations[i] = get_duration(&start, &end);
  }
}

static void do_cleanup(const benchmark_config_t *config, benchmark_state_t *state) {
  close_or_die(state->fd);
  if (config->delete_afterwards) {
    remove_or_die(config->filepath);
  }
}

/** Wrapper-function.
 *
 * The only non-static function. Creates the state and wraps the benchmark.
 */
benchmark_results_t *benchmark_file(const benchmark_config_t *config)
{
  benchmark_state_t state;
  benchmark_results_t *results = malloc_or_die(sizeof(benchmark_results_t));

  if (config->drop_cache_first)
    drop_page_cache();
  init_state(config, &state);
  init_file(config, &state);
  init_results(config, results);

  do_benchmark(config, &state, results);

  do_cleanup(config, &state);
  return results;
}

