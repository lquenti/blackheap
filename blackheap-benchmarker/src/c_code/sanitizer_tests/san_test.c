#include "../benchmarker.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
  /* some cfg */
  struct benchmark_config config = {
    .filepath = "/tmp/test_file.bin",
    .memory_buffer_in_bytes = 1024,
    .file_size_in_bytes = 1024 * 10,
    .access_size_in_bytes = 128,
    .number_of_io_op_tests = 10,
    .access_pattern_in_memory = ACCESS_PATTERN_SEQUENTIAL,
    .access_pattern_in_file = ACCESS_PATTERN_SEQUENTIAL,
    .is_read_operation = true,
    .prepare_file_size = true,
    .drop_cache_first = false,
    .do_reread = false,
    .restrict_free_ram_to = 0
  };

  struct benchmark_results results = benchmark_file(&config);

  if (results.res == ERROR_CODES_SUCCESS) {
   printf("Benchmark completed successfully.\n");
   printf("Results length: %zu\n", results.length);
   /* Print a few result durations */
   for (size_t i = 0; i < results.length && i < 3; ++i) {
      printf("Duration for operation %zu: %f seconds\n", i, results.durations[i]);
   }
  } else {
    printf("Benchmark failed with error code: %d\n", results.res);
  }

  if (results.durations != NULL) {
    free(results.durations);
  }

  return 0;
}

