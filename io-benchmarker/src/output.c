//
// Created by lquenti on 02.12.21
//


#include <stdio.h>
#include <assert.h>

#include "output.h"

// TODO move me
static inline char *access_pattern_to_str(access_pattern_t pat)
{
  switch (pat)
  {
  case ACCESS_PATTERN_CONST:
    return "const";
  case ACCESS_PATTERN_SEQUENTIAL:
    return "seq";
  case ACCESS_PATTERN_RANDOM:
    return "rnd";
  }
  /* This should never happen */
  __builtin_unreachable();
}

static void print_config(const benchmark_config_t *config)
{
  fprintf(stdout, "\"%s\": \"%s\",\n", "filepath", config->filepath);
  fprintf(stdout, "\"%s\": %zu,\n", "repeats", config->number_of_io_op_tests);
  fprintf(stdout, "\"%s\": %zu,\n", "memory_buffer_in_bytes", config->memory_buffer_in_bytes);
  fprintf(stdout, "\"%s\": %zu,\n", "file_size_in_bytes", config->file_size_in_bytes);
  fprintf(stdout, "\"%s\": %zu,\n", "access_size_in_bytes", config->access_size_in_bytes);
  fprintf(stdout, "\"%s\": \"%s\",\n", "access_pattern_in_memory", access_pattern_to_str(config->access_pattern_in_memory));
  fprintf(stdout, "\"%s\": \"%s\",\n", "access_pattern_in_file", access_pattern_to_str(config->access_pattern_in_file));
  fprintf(stdout, "\"%s\": \"%s\",\n", "io_operation", config->is_read_operation ? "read" : "write");
  fprintf(stdout, "\"%s\": %s,\n", "prepare_file_size", config->prepare_file_size ? "true" : "false");
  fprintf(stdout, "\"%s\": %zu,\n", "restricted_ram_in_bytes", config->restrict_free_ram_to);
  fprintf(stdout, "\"%s\": %s,\n", "use_o_direct", config->use_o_direct ? "true" : "false");
  fprintf(stdout, "\"%s\": %s,\n", "drop_cache_first", config->drop_cache_first ? "true" : "false");
  fprintf(stdout, "\"%s\": %s,\n", "reread_every_block", config->do_reread ? "true" : "false");
  fprintf(stdout, "\"%s\": %s,\n", "delete_afterwards", config->delete_afterwards ? "true" : "false");
}

static void print_results(const benchmark_results_t *results)
{
  fprintf(stdout, "\"durations\": [\n");
  for (size_t i = 0; i < results->length; ++i)
  {
    fprintf(stdout, "\t%.17g%s\n", results->durations[i], (i == results->length - 1) ? "" : ",");
  }
  fprintf(stdout, "]\n");
}

// TODO: make stdout variable via parameter
// TODO: Version mitgeben
void print_output(const benchmark_config_t *config, const benchmark_results_t *results)
{
  fprintf(stdout, "{\n");
  print_config(config);
  print_results(results);
  fprintf(stdout, "}\n");
}

