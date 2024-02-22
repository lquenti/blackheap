#include"./benchmarker_internal.h"
#include "benchmarker.h"

#include<errno.h>
#include<stdbool.h>
#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<time.h>
#include<unistd.h>

enum error_codes drop_page_cache() {
  /* sync first */
  sync();

  int fd = open(DROP_PAGE_CACHE, O_WRONLY);
  if (fd == -1) {
    if (errno == EACCES) {
      fprintf(stderr, "In order to drop the page cache, we need permissions to open" DROP_PAGE_CACHE "\n");
      return ERROR_CODES_DROP_PAGE_CACHE_FAILED_NO_PERMISSIONS;
    } else {
      fprintf(stderr, "Unknown Error while opening" DROP_PAGE_CACHE ".\nError: %s\n", strerror(errno));
      return ERROR_CODES_DROP_PAGE_CACHE_FAILED_OTHER;
    }
  }
  
  char magic_value = '3';
  ssize_t res = write(fd, &magic_value, sizeof(char));
  if (res == -1) {
      fprintf(stderr, "Dropping the page cache failed. The write was not successful.\nError: %s\n", strerror(errno));
    return ERROR_CODES_DROP_PAGE_CACHE_FAILED_OTHER;
  }

  /* in case the OS does it non-blockingly */
  sleep(5);

  close(fd);
  return ERROR_CODES_SUCCESS;
}

enum error_codes init_state(const struct benchmark_config *config, struct benchmark_state *state) {
  void *ptr;

  ptr = malloc(config->memory_buffer_in_bytes);
  if (ptr == NULL) {
      fprintf(stderr, "Mallocing the big memory buffer of size %zu failed\n", config->memory_buffer_in_bytes);
      return ERROR_CODES_MALLOC_FAILED;
  }
  /* enforce that the buffer actually exists */
  memset(ptr, '1', (unsigned long)config->memory_buffer_in_bytes);
  state->buffer = ptr;
  state->last_mem_offset = 0;
  state->last_file_offset = 0;

  if (config->is_read_operation) {
    state->io_op = read;
  } else {
    /* just casting away the const for the void pointer */
    state->io_op = (ssize_t (*)(int, void *, size_t))write;
  }

  return ERROR_CODES_SUCCESS;
}

enum error_codes init_file(const struct benchmark_config *config, struct benchmark_state *state) {
  /* is it externally managed? */
  if (!config->prepare_file_size) {
    return ERROR_CODES_SUCCESS;
  }

  /* try to open it */
  state->fd = open(config->filepath, O_CREAT | O_RDWR, 0644); 
  if (state->fd == -1) {
    fprintf(stderr, "Error opening \"%s\".\nError: %s\n", config->filepath, strerror(errno));
    return ERROR_CODES_OPEN_FAILED;
  }

  /* Does it already have the correct size */
  struct stat st;
  int res = fstat(state->fd, &st);
  close(state->fd);
  if (res == -1) {
    fprintf(stderr, "Error checking file size of %s\nError: %s\n", config->filepath, strerror(errno));
    return ERROR_CODES_FSTAT_FAILED;
  }
  if ((size_t)st.st_size == config->file_size_in_bytes) {
    return ERROR_CODES_SUCCESS;
  }

  /* If not, we first truncate it to zero */
  state->fd = open(config->filepath, O_RDWR | O_TRUNC, 0644);
  if (state->fd == -1) {
    fprintf(stderr, "Error opening \"%s\".\nError: %s\n", config->filepath, strerror(errno));
    return ERROR_CODES_OPEN_FAILED;
  }

  /* 64k is a good write size */
  const size_t block_size = 64*1024;
  size_t bytes_written = 0;
  ssize_t write_result;

  /* Fill bytes with 1s */
  while (bytes_written < config->file_size_in_bytes) {
    size_t bytes_to_write = config->file_size_in_bytes - bytes_written;
    if (bytes_to_write > block_size) {
      bytes_to_write = block_size;
    }

    write_result = write(state->fd, state->buffer, bytes_to_write);
    if (bytes_to_write == -1) {
      fprintf(stderr, "Failed to write to \"%s\"\nError: %s\n", config->filepath, strerror(errno));
      close(state->fd);
      return ERROR_CODES_WRITE_FAILED;
    }
    bytes_written += write_result;
  }

  /* Check whether it worked */
  if (fsync(state->fd) == -1) {
    fprintf(stderr, "Failed to flush \"%s\" to disk.\nError: %s\n", config->filepath, strerror(errno));
    close(state->fd);
    return ERROR_CODES_FSYNC_FAILED;
  }

  if (fstat(state->fd, &st) == -1) {
    fprintf(stderr, "Error checking file size of %s\nError: %s\n", config->filepath, strerror(errno));
    close(state->fd);
    return ERROR_CODES_FSTAT_FAILED;
  }

  close(state->fd);

  if (st.st_size != config->file_size_in_bytes) {
    fprintf(
      stderr, 
      "Incorrect file size after filling \"%s\". Expected: %zu Actual: %lld\n",
      config->filepath,
      config->file_size_in_bytes,
      (long long)st.st_size
    );
    return ERROR_CODES_INCORRECT_FILE_BUFFER_SIZE;
  }

  return ERROR_CODES_SUCCESS;
}


enum error_codes init_results(const struct benchmark_config *config, struct benchmark_results *results) {
  results->res = ERROR_CODES_SUCCESS;
  results->length = config->number_of_io_op_tests;

  results->durations = malloc(sizeof(double) * config->number_of_io_op_tests);
  return (results->durations == NULL) ? ERROR_CODES_MALLOC_FAILED : ERROR_CODES_SUCCESS;
}

enum error_codes reread(const struct benchmark_config *config, const struct benchmark_state *state) {
  int res = state->io_op(state->fd, state->buffer, config->access_size_in_bytes);
  if (res == -1) {
    fprintf(stderr, "Failed to write to \"%s\"\nError: %s\n", config->filepath, strerror(errno));
    return ERROR_CODES_WRITE_FAILED;
  }

  /* Seek back so that we read it twice */
  off_t seek_res = lseek(state->fd, state->last_file_offset, SEEK_SET);
  if (seek_res == -1) {
    fprintf(stderr, "Failed to seek \"%s\" to %zu \nError: %s\n", config->filepath, state->last_file_offset, strerror(errno));
    return ERROR_CODES_LSEEK_FAILED;
  }

  return ERROR_CODES_SUCCESS;
}

double timespec_to_double(const struct timespec *time) {
  return time->tv_sec + 0.001 * 0.001 * 0.001 * time->tv_nsec;
}

void pick_next_mem_position(const struct benchmark_config *config, struct benchmark_state *state) {
  switch (config->access_pattern_in_memory) {
    case ACCESS_PATTERN_CONST:
      /* After one io-op the pointer does not get moved like the fd-state for the file */
      return;
    case ACCESS_PATTERN_SEQUENTIAL:
      state->last_mem_offset += config->access_size_in_bytes;
      return;
    case ACCESS_PATTERN_RANDOM:
      state->last_mem_offset = ((size_t)rand() * 128) % (config->memory_buffer_in_bytes - config->access_size_in_bytes);
      return;
  }
}

enum error_codes pick_next_file_position(const struct benchmark_config *config, struct benchmark_state *state) {
  switch (config->access_pattern_in_file) {
    case ACCESS_PATTERN_CONST: {
        /* Update file descriptor */
        off_t new_offset = lseek(state->fd, 0, SEEK_SET);
        if (new_offset == -1) {
          fprintf(stderr, "Failed to seek \"%s\" to 0. \nError: %s\n", config->filepath, strerror(errno));
          return ERROR_CODES_LSEEK_FAILED;
        }
      }
      break;
    case ACCESS_PATTERN_SEQUENTIAL: {
        /* update state */
        state->last_file_offset += config->access_size_in_bytes;
        
        /* Check if we have to wrap */
        if (state->last_file_offset + config->access_size_in_bytes > config->file_size_in_bytes) {
          /* Lets start at zero again */
          state->last_file_offset = 0;
          
          off_t new_offset = lseek(state->fd, 0, SEEK_SET);
          if (new_offset == -1) {
            fprintf(stderr, "Failed to seek \"%s\" to 0. \nError: %s\n", config->filepath, strerror(errno));
            return ERROR_CODES_LSEEK_FAILED;
          }
        }
      }
      break;
    case ACCESS_PATTERN_RANDOM: {
        size_t new_file_pos = ((size_t)rand() * 128) % (config->file_size_in_bytes - config->access_size_in_bytes);

        /* Update state */
        state->last_file_offset = new_file_pos;

        /* Update file descriptor */
        off_t new_offset = lseek(state->fd, new_file_pos, SEEK_SET);
        if (new_offset == -1) {
          fprintf(stderr, "Failed to seek \"%s\" to %zu. \nError: %s\n", config->filepath, (size_t)new_offset, strerror(errno));
          return ERROR_CODES_LSEEK_FAILED;
        }
      }
      break;
  }
  return ERROR_CODES_SUCCESS;
}

enum error_codes do_benchmark(const struct benchmark_config *config, struct benchmark_state *state, struct benchmark_results *results) {
  struct timespec start, end;
  int res;
  enum error_codes ret = ERROR_CODES_SUCCESS;

  for (size_t i=0; i<config->number_of_io_op_tests; ++i) {
    if (config->do_reread) {
      ret = reread(config, state);
      if (ret != ERROR_CODES_SUCCESS) {
        return ret;
      }
    }

    /* Do the operation */
    clock_gettime(CLOCK_MONOTONIC, &start);
    res = state->io_op(state->fd, state->buffer, config->access_size_in_bytes);
    clock_gettime(CLOCK_MONOTONIC, &end);

    /* did it work? */
    if (res != -1) {
      results->durations[i] = timespec_to_double(&end) - timespec_to_double(&start);
    } else {
      results->durations[i] = -1.0;
    }

    /* update offsets */
    pick_next_mem_position(config, state);
    ret = pick_next_file_position(config, state);
    if (ret != ERROR_CODES_SUCCESS) {
      return ret;
    }
  }

  return ERROR_CODES_SUCCESS;
}


void do_cleanup(const struct benchmark_config *config, struct benchmark_state *state) {
  close(state->fd);
  free(state->buffer);
}

struct benchmark_results benchmark_file(const struct benchmark_config *config) {
  struct benchmark_state state;
  struct benchmark_results results;
  results.res = ERROR_CODES_SUCCESS;

  /* init randomness */
  srand((unsigned int)time(NULL));
  
  /* Drop page cache if set (note that this requires root) */
  if (config->drop_cache_first) {
    results.res = drop_page_cache();
  }

  /* Init memory buffer and other state */
  if (results.res == ERROR_CODES_SUCCESS) {
    results.res = init_state(config, &state);
  }
  
  /* init file buffer */
  if (results.res == ERROR_CODES_SUCCESS) {
    results.res = init_file(config, &state);
  }

  /* Init results array */
  if (results.res == ERROR_CODES_SUCCESS) {
    results.res = init_results(config, &results);
  }

  /* Do the benchmark! */

  /* cleanup */
  do_cleanup(config, &state);
  
  return results;
}
