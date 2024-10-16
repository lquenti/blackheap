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
  
  /* check kernel docs */
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

  /* 64k is a good write size (if our buffer is big enough) */
  size_t block_size = 64*1024;
  if (block_size > config->memory_buffer_in_bytes) {
    block_size = config->memory_buffer_in_bytes;
  }

  size_t bytes_written = 0;
  ssize_t write_result;

  /* Fill bytes with 1s */
  while (bytes_written < config->file_size_in_bytes) {
    size_t bytes_to_write = config->file_size_in_bytes - bytes_written;
    if (bytes_to_write > block_size) {
      bytes_to_write = block_size;
    }

    write_result = write(state->fd, state->buffer, bytes_to_write);
    if (write_result == -1) {
      fprintf(stderr, "Failed to write to \"%s\"\nError: %s\n", config->filepath, strerror(errno));
      close(state->fd);
      return ERROR_CODES_WRITE_FAILED;
    }
    bytes_written += write_result;
  }

  if (fsync(state->fd) == -1) {
    fprintf(stderr, "Failed to flush \"%s\" to disk.\nError: %s\n", config->filepath, strerror(errno));
    close(state->fd);
    return ERROR_CODES_FSYNC_FAILED;
  }

  /* Check whether it worked */
  if (fstat(state->fd, &st) == -1) {
    fprintf(stderr, "Error checking file size of %s\nError: %s\n", config->filepath, strerror(errno));
    close(state->fd);
    return ERROR_CODES_FSTAT_FAILED;
  }

  close(state->fd);

  if ((long long)st.st_size != (long long)config->file_size_in_bytes) {
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

long parse_from_meminfo(char *key) {
  long res = -1;
  size_t keylen = strlen(key);

  FILE *fp = fopen(MEMINFO, "r");
  if (!fp) {
    perror("Failed to open MEMINFO");
    return res;
  }

  char buf[100];
  while (fgets(buf, sizeof(buf), fp)) {

    /* is it not out match? */
    if (strncmp(buf, key, keylen) != 0) {
      continue;
    }
    printf("%s\n", buf);

    /* It is out match. */
    char *colon = strchr(buf, ':');
    if (colon) {
      res = atol(colon+1);
      break;
    }
  }

  fclose(fp);
  return res;
}

size_t get_available_mem_kib() {
  long free = parse_from_meminfo("MemFree");
  long cached = parse_from_meminfo("Cached");
  long buffers = parse_from_meminfo("Buffers");

  /* Log if any of them failed... */
  if (free == -1) {
    fprintf(stderr, "Reading \"MemFree\" from /proc/meminfo failed...");
    return -1;
  }
  if (cached == -1) {
    fprintf(stderr, "Reading \"Cached\" from /proc/meminfo failed...");
    return -1;
  }
  if (buffers == -1) {
    fprintf(stderr, "Reading \"Buffers\" from /proc/meminfo failed...");
    return -1;
  }

  return free+cached+buffers;
}

/* Note that the callee has to free if it succeeded */
struct allocation_result allocate_memory_until(size_t space_left_in_kib) {
  struct allocation_result result;
  result.pointers = NULL;
  result.length = 0;
  
  bool was_successful = true;

  size_t current_available = get_available_mem_kib();
  while (current_available > space_left_in_kib) {
    size_t delta = current_available - space_left_in_kib;
    size_t n = (delta < 128 ? delta : 128) * 1024;

    void *p = malloc(n);
    if (!p) {
      fprintf(stderr, "Mallocing %zu bytes to restrict the memory failed. Currently still available: %zu KiB\n", n, current_available);
      was_successful = false;
      break;
    }

    /* Ensure the memory is allocated */
    memset(p, '1', n); 

    /* add to ptrs */
    void **new_pointers = realloc(result.pointers, (result.length + 1) * sizeof(void *));
    if (!new_pointers) {
      fprintf(stderr, "Reallocating pointers array failed. Current length: %zu\n", result.length);
      /* free the last allocation */
      free(p);
      break;
    }

    result.pointers = new_pointers;
    result.pointers[result.length] = p;
    result.length++;

    current_available = get_available_mem_kib();
  }

  /* If it failed, we will clean up... */
  if (!was_successful) {
    for (ssize_t i=0; i<result.length; ++i) {
      free(result.pointers[i]);
    }
    free(result.pointers);
    result.pointers = NULL;
    result.length = -1;
  }

  return result;
}


enum error_codes reread(const struct benchmark_config *config, const struct benchmark_state *state) {
  int res = state->io_op(state->fd, state->buffer, config->access_size_in_bytes);
  if (res == -1) {
    fprintf(stderr, "Failed to write to \"%s\"\nError: %s\n", config->filepath, strerror(errno));
    return config->is_read_operation ? ERROR_CODES_READ_FAILED : ERROR_CODES_WRITE_FAILED;
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

      /* Check if we have to wrap */
      if (state->last_mem_offset + config->access_size_in_bytes > config->memory_buffer_in_bytes) {
        state->last_mem_offset = 0;
      }
      return;
    case ACCESS_PATTERN_RANDOM:
      state->last_mem_offset = ((size_t)rand() * 128) % (config->memory_buffer_in_bytes - config->access_size_in_bytes);
      return;
    case ACCESS_PATTERN_REVERSE: {
      /* we only have to move one back since it didnt update since the last read. */

      /* Check for wrapping */
      if (state->last_mem_offset < config->access_size_in_bytes) {
        state->last_mem_offset = config->memory_buffer_in_bytes - config->access_size_in_bytes;
      } else {
        state->last_mem_offset -= config->access_size_in_bytes;
      }
      return;
    }
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
    case ACCESS_PATTERN_REVERSE: { 
      /* two access sizes since we need one to go back to the last read, and one more to go backwards */

      /* check for wrapping */
      if (state->last_file_offset < 2 * config->access_size_in_bytes) {
        /* Do we even have enough space to move back 2 access sizes? */
        if (config->file_size_in_bytes > 2 * config->access_size_in_bytes) {
          state->last_file_offset = config->file_size_in_bytes - config->access_size_in_bytes;
        } else {
          fprintf(stderr, "File size %zu is too small for reverse access pattern with %zu access size.\n", config->file_size_in_bytes, config->access_size_in_bytes);
          return ERROR_CODES_TOO_SMALL_FILE_BUFFER;
        }
      } else {
        state->last_file_offset -= 2 * config->access_size_in_bytes;
      }

      /* Update file descriptor */
      off_t new_offset = lseek(state->fd, state->last_file_offset, SEEK_SET);
      if (new_offset == -1) {
        fprintf(stderr, "Failed to seek \"%s\" to 0. \nError: %s\n", config->filepath, strerror(errno));
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
  struct allocation_result mallocs;
  /* to make clang happy */
  mallocs.pointers = NULL;
  mallocs.length = 0;

  /* Open fd (closed by cleanup) */
  state->fd = open(config->filepath, O_RDWR, 0644); 
  if (state->fd == -1) {
    fprintf(stderr, "Error opening \"%s\".\nError: %s\n", config->filepath, strerror(errno));
    return ERROR_CODES_OPEN_FAILED;
  }

  /* restrict memory if configured */
  if (config->restrict_free_ram_to != 0) {
    mallocs = allocate_memory_until(config->restrict_free_ram_to/1024);
    if (mallocs.length == -1) {
        return ERROR_CODES_MALLOC_FAILED;
    }
  }

  for (size_t i=0; i<config->number_of_io_op_tests; ++i) {
    if (config->do_reread) {
      ret = reread(config, state);
      if (ret != ERROR_CODES_SUCCESS) {
        goto cleanup_do_benchmark;
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
      // Note: this implies that whenever **a single** I/O op fails the whole
      // benchmark is seen as unsuccessful.
      //
      // This may not be optimal, although it is currently unclear to me how
      // a "expectable amount" of failing IO ops would look like...
      //
      // TODO thus we crash for now, but I am very open for a PR with better ideas
      results->durations[i] = -1.0;
      return config->is_read_operation ? ERROR_CODES_READ_FAILED : ERROR_CODES_WRITE_FAILED;
    }

    /* update offsets */
    pick_next_mem_position(config, state);
    ret = pick_next_file_position(config, state);
    if (ret != ERROR_CODES_SUCCESS) {
      goto cleanup_do_benchmark;
    }
  }

cleanup_do_benchmark:
  if (config->restrict_free_ram_to != 0) {
    for (ssize_t i=0; i<mallocs.length; ++i) {
      free(mallocs.pointers[i]);
    }
    free(mallocs.pointers);
  }
  return ret;
}


void do_cleanup(struct benchmark_state *state) {
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
  if (results.res == ERROR_CODES_SUCCESS) {
    do_benchmark(config, &state, &results);
  }

  /* cleanup */
  do_cleanup(&state);
  
  return results;
}
