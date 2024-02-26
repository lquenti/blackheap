#include "../benchmarker.h"
#include <stdio.h>
#include <stdlib.h>

void run_benchmark(struct benchmark_config config, const char *description) {
    printf("Running benchmark: %s\n", description);
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
    printf("\n");
}

int main() {
    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024,
        .file_size_in_bytes = 1024 * 10,
        .access_size_in_bytes = 128,
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_CONST,
        .access_pattern_in_file = ACCESS_PATTERN_CONST,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Simple Test (const)");

    run_benchmark((struct benchmark_config){
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
    }, "Simple Test (seq)");
  
    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024,
        .file_size_in_bytes = 1024 * 10,
        .access_size_in_bytes = 128,
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_RANDOM,
        .access_pattern_in_file = ACCESS_PATTERN_RANDOM,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Simple Test (rnd)");

    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024,
        .file_size_in_bytes = 1024 * 10,
        .access_size_in_bytes = 128,
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_REVERSE,
        .access_pattern_in_file = ACCESS_PATTERN_REVERSE,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Simple Test (rev)");

    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024 * 1024 * 512, // 512MB
        .file_size_in_bytes = 1024 * 1024 * 1024, // 1GB
        .access_size_in_bytes = 1024 * 1024 * 10, // 10MB
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_SEQUENTIAL,
        .access_pattern_in_file = ACCESS_PATTERN_SEQUENTIAL,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Handle Large Files");

    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024 * 512, // 512KB
        .file_size_in_bytes = 1024 * 512, // 512KB
        .access_size_in_bytes = 1024 * 300, // 300KB
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_SEQUENTIAL,
        .access_pattern_in_file = ACCESS_PATTERN_SEQUENTIAL,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Can it handle wrapping (seq)");

    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024 * 512, // 512KB
        .file_size_in_bytes = 1024 * 512, // 512KB
        .access_size_in_bytes = 1024 * 300, // 300KB
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_RANDOM,
        .access_pattern_in_file = ACCESS_PATTERN_RANDOM,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Can it handle wrapping (rnd)");

    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024 * 512, // 512KB
        .file_size_in_bytes = 1024 * 512, // 512KB
        .access_size_in_bytes = 1024 * 300, // 300KB
        .number_of_io_op_tests = 10,
        .access_pattern_in_memory = ACCESS_PATTERN_REVERSE,
        .access_pattern_in_file = ACCESS_PATTERN_REVERSE,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Can it handle wrapping (rev)");

    run_benchmark((struct benchmark_config){
        .filepath = "/tmp/test_file.bin",
        .memory_buffer_in_bytes = 1024, // 1KB
        .file_size_in_bytes = 1024 * 10, // 10KB
        .access_size_in_bytes = 1, // 1 byte
        .number_of_io_op_tests = 100000, // A lot of accesses
        .access_pattern_in_memory = ACCESS_PATTERN_SEQUENTIAL,
        .access_pattern_in_file = ACCESS_PATTERN_SEQUENTIAL,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Many access sizes (test asan for leaks)");

    run_benchmark((struct benchmark_config){
        .filepath = "/dev/shm/test_file.bin",
        .memory_buffer_in_bytes = 1024 * 1024, // 1MB
        .file_size_in_bytes = 1024 * 1024 * 10, // 10MB
        .access_size_in_bytes = 1024 * 10, // 10KB
        .number_of_io_op_tests = 100, // Moderate number of accesses
        .access_pattern_in_memory = ACCESS_PATTERN_SEQUENTIAL,
        .access_pattern_in_file = ACCESS_PATTERN_SEQUENTIAL,
        .is_read_operation = true,
        .prepare_file_size = true,
        .drop_cache_first = false,
        .do_reread = false,
        .restrict_free_ram_to = 0
    }, "Memory as filesystem with /dev/shm");

    remove("/tmp/test_file.bin");
    remove("/dev/shm/test_file.bin");
    return 0;
}

