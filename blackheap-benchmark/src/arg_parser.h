#ifndef IO_BENCHMARK_ARG_PARSER_H
#define IO_BENCHMARK_ARG_PARSER_H

#include "io_benchmark.h"
#include "helper.h"

const char *DEFAULT_FILENAME;
const char *DEFAULT_FILEPATH;
const char *argp_program_bug_address;
const char *argp_program_version;


typedef struct benchmark_config_parser_t
{
  char *filepath;
  size_t memory_buffer_in_bytes;
  size_t file_size_in_bytes;
  size_t access_size_in_bytes;
  size_t number_of_io_op_tests;
  size_t available_bytes;
  access_pattern_t access_pattern_in_memory;
  access_pattern_t access_pattern_in_file;
  bool is_read_operation;
  bool prepare_file_size;
  bool read_was_selected;
  bool write_was_selected;
  size_t free_ram_if_selected;
  bool o_direct_was_selected;
  bool drop_cache_was_selected;
  bool reread_was_selected;
  bool delete_afterwards_was_selected;
  size_t number_of_missing_arugments;
} benchmark_config_parser_t;

typedef enum argp_index_values_t
{
  GROUP_BENCHMARK_OPTIONS = 2000,

  GROUP_BENCHMARK_OPERATIONS = 2100,

  GROUP_ACCESS_PATTERNS = 2200,
  ID_MEM_PATTERN = 2201,
  ID_FILE_PATTERN = 2202,

  GROUP_OTHER_PARAMETERS = 2300,
  ID_FILEPATH = 2301,
  ID_REPEATS = 2302,
  ID_MEM_SIZE = 2303,
  ID_FILE_SIZE = 2304,
  ID_ACCESS_SIZE = 2305,

  GROUP_TOGGLES = 2400,
  ID_RESTRICT_FREE_RAM = 2401,
  ID_USE_O_DIRECT = 2402,
  ID_DROP_CACHE = 2403,
  ID_DO_REREAD = 2404,
  ID_DELETE_AFTERWARDS = 2405,
} argp_index_values_t;

void parse_benchmark_arguments(int argc, char **argv, benchmark_config_t *out);


#endif //IO_BENCHMARK_ARG_PARSER_H
