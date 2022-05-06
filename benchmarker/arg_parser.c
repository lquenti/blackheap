//
// Created by lquenti on 02.12.21.
//


#include <ctype.h>
#include <argp.h>
#include <stdbool.h>
#include<string.h>
#include<stdlib.h>

#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>

#include"arg_parser.h"

/* argp uses cool default indexing when one does not define some values.
 * Thus, this is okay (and intended) in this file.
 */
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"


const char *DEFAULT_FILENAME = "io_benchmark_test_file.dat";
const char *DEFAULT_FILEPATH = "/tmp/io_benchmark_test_file.dat";
const char *argp_program_bug_address = "https://gitlab.gwdg.de/lars.quentin1/io-benchmark/-/issues";
const char *argp_program_version = "0.1.0";


// TODO put me in helper
static inline void to_lower_str(char *p)
{
  for (; *p; ++p)
  {
    *p = tolower(*p);
  }
}

static inline access_pattern_t parse_access_pattern(char *p, struct argp_state *state)
{
  to_lower_str(p);
  if (strcmp(p, "off0") == 0)
  {
    return ACCESS_PATTERN_CONST;
  }
  else if (strcmp(p, "seq") == 0)
  {
    return ACCESS_PATTERN_SEQUENTIAL;
  }
  else if (strcmp(p, "rnd") == 0)
  {
    return ACCESS_PATTERN_RANDOM;
  }
  argp_error(state, "Unknown access pattern '%s'\n", p);
  exit(1);
}

static inline void get_size_t_or_fail(const char *str, size_t *out, struct argp_state *state)
{
  int ret = sscanf(str, "%zu", out);
  if (ret == EOF)
  {
    argp_error(state, "Could not read parameters; EOF encountered while parsing number '%s' with error '%s'.\n",
               str,
               strerror(errno));
  }
  else if (ret == 0)
  {
    argp_error(state, "Could not read parameters; Could not convert number '%s'\n", str);
  }
}

/* This function does 2 things:
 * 1. We have to check whether we have a normal file or not.
 *    If we do not have a normal file, we probably can't just
 *    write into it to change it's size.
 *    By normal files, I mean things like:
 *    - Character special devices
 *    - Block special devices
 *    - Pipes
 *    See: https://www.gnu.org/software/libc/manual/html_node/Testing-File-Type.html
 * 2. If they, by mistake or not, set the filepath to a directory, we create a file into
 *    that directory.
 */
// TODO break me up
static bool is_normal_file(char **path_ptr)
{
  struct stat st;
  int res = stat(*path_ptr, &st);

  /* if it failed, it either does not exist or it REALLY failed. */
  if (res == -1)
  {
    if (errno != ENOENT)
    {
      fprintf(stderr, "ERROR: %s is an invalid path. Please check permissions.\n", *path_ptr);
      exit(1);
    }
    /* So it was just non-existing */
    int fd = open_or_die(*path_ptr, O_CREAT | O_WRONLY, 0644);
    close_or_die(fd);
    /* We know it's a file, thus we are good */
    return true;
  }

  /* The stat request worked. If it is a file, we are good */
  if (S_ISREG(st.st_mode))
  {
    return true;
  }
  /* If it is a directory, let's create a file within it */
  if (S_ISDIR(st.st_mode))
  {
    /* I know, I know this is formally a memory leak
     * but I couldn't be less bothered because
     * argvs are not freed anyways
     */
    char *buffer = malloc(sizeof(char) * 128);
    strcpy(buffer, *path_ptr);
    strcat(buffer, "/");
    strcat(buffer, DEFAULT_FILENAME);
    int fd = open_or_die(buffer, O_CREAT | O_WRONLY, 0644);
    close_or_die(fd);
    /* Here we are changing it to a file.
     * So it looks like we never had a wrong input :)
     */
    *path_ptr = buffer;
    return true;
  }
  /* It exists and is neither a file nor a directory, we have to be careful. */
  return false;
}

static inline void check_argument_numbers(const benchmark_config_parser_t *config, struct argp_state *state)
{
  if (config->number_of_missing_arugments != 0)
    argp_error(state, "Wrong number of arguments:\n");
}
static inline void check_read_and_write(const benchmark_config_parser_t *config, struct argp_state *state)
{
  if (config->read_was_selected && config->write_was_selected)
    argp_error(state, "Both --read and --write were selected");
  if (!(config->read_was_selected || config->write_was_selected))
    argp_error(state, "Neither --read and --write were selected");
}
/**
 * Both strategies have the goal that the vfs page cache is bypassed.
 * Thus, it doesn't make sense if both are activated.
 */
static inline void check_both_memory_restriction_were_set(const benchmark_config_parser_t *config, struct argp_state *state) {
  if (config->o_direct_was_selected && config->free_ram_if_selected != 0)
    argp_error(state, "Selecting both --free-ram and --o-direct will screw the results");
}
static inline void check_access_size_larger_mem_buf(const benchmark_config_parser_t *config, struct argp_state *state)
{
  if (config->access_size_in_bytes > config->memory_buffer_in_bytes)
    argp_error(state, "The access size of a single request can't be larger than than the whole memory buffer");
}
static inline void check_access_size_larger_file_buf(const benchmark_config_parser_t *config, struct argp_state *state)
{
  if (config->access_size_in_bytes > config->file_size_in_bytes)
    argp_error(state, "The access size of a single request can't be larger than than the whole file size");
}

/** Check whether we can sequentially read without wrapping.
 *
 * The big advantage of sequential reads (compared to let's say random ones) is that the
 * kernel can prefetch accordingly.
 *
 * Let's say that we have n tests. The access size of each I/O read is b Bytes.
 * Let's now say that either the file size is smaller than n*b. Then we have to
 * start at the beginning, thus creating a spike.
 *
 * Storage is not scarce anymore, especially in our access sizes.
 */
static inline void check_needs_to_wrap(const benchmark_config_parser_t *config, struct argp_state *state)
{
  size_t size_needed = config->access_size_in_bytes * config->number_of_io_op_tests;
  bool use_prefetching = config->access_pattern_in_file == ACCESS_PATTERN_SEQUENTIAL;
  if (use_prefetching && size_needed > config->file_size_in_bytes)
    argp_error(state, "When reading sequentially, we need a file buffer size of at least access_size * number_of_tests bytes");
}

static inline void check_does_not_try_to_delete_special_file(const benchmark_config_parser_t *config, struct argp_state *state)
{
  if ((!config->prepare_file_size) && config->delete_afterwards_was_selected)
    argp_error(state, "You can't delete a special file like '%s'\n--delete-afterwards is invalid here",
        config->filepath);
}

static error_t parse_opt(int key, char *arg, struct argp_state *state)
{
  benchmark_config_parser_t *config = state->input;

  switch (key)
  {
  case 'r':
    config->is_read_operation = true;
    config->read_was_selected = true;
    break;
  case 'w':
    config->is_read_operation = false;
    config->write_was_selected = true;
    break;
  case ID_MEM_PATTERN:
    config->access_pattern_in_file = parse_access_pattern(arg, state);
    config->number_of_missing_arugments--;
    break;
  case ID_FILE_PATTERN:
    config->access_pattern_in_memory = parse_access_pattern(arg, state);
    config->number_of_missing_arugments--;
    break;
  case ID_FILEPATH:
    config->filepath = arg;
    config->prepare_file_size = is_normal_file(&config->filepath);
    break;
  case ID_REPEATS:
    get_size_t_or_fail(arg, &config->number_of_io_op_tests, state);
    config->number_of_missing_arugments--;
    break;
  case ID_MEM_SIZE:
    get_size_t_or_fail(arg, &config->memory_buffer_in_bytes, state);
    config->number_of_missing_arugments--;
    break;
  case ID_FILE_SIZE:
    get_size_t_or_fail(arg, &config->file_size_in_bytes, state);
    config->number_of_missing_arugments--;
    break;
  case ID_ACCESS_SIZE:
    get_size_t_or_fail(arg, &config->access_size_in_bytes, state);
    config->number_of_missing_arugments--;
    break;
  case ID_RESTRICT_FREE_RAM:
    get_size_t_or_fail(arg, &config->free_ram_if_selected, state);
    break;
  case ID_USE_O_DIRECT:
    config->o_direct_was_selected = true;
    break;
  case ID_DROP_CACHE:
    config->drop_cache_was_selected = true;
    break;
  case ID_DO_REREAD:
    config->reread_was_selected = true;
    break;
  case ID_DELETE_AFTERWARDS:
    config->delete_afterwards_was_selected = true;
    break;
  case ARGP_KEY_END:
    check_argument_numbers(config, state);
    check_read_and_write(config, state);
    check_both_memory_restriction_were_set(config, state);
    check_access_size_larger_mem_buf(config, state);
    check_access_size_larger_file_buf(config, state);
    check_needs_to_wrap(config, state);
    check_does_not_try_to_delete_special_file(config, state);
    break;
  }
  return 0;
}

static struct argp_option options[] = {
    /* Define a group for all io-benchmark options in case someone wants to embed them. */
    {0, 0, 0, 0, "Benchmark Options:", GROUP_BENCHMARK_OPTIONS},

    /* Read and write are mutually exclusive. */
    {0, 0, 0, 0, "Possible Benchmark Operations:", GROUP_BENCHMARK_OPERATIONS},
    {"read", 'r', 0, 0, "Benchmark a read operation."},
    {"write", 'w', 0, 0, "Benchmark a write operation."},

    /* File Patterns */
    {0, 0, 0, 0, "Access Patterns:\nPossible MODES: off0,seq,rnd", GROUP_ACCESS_PATTERNS},
    {"mem-pattern", ID_MEM_PATTERN, "MODE", 0, "Pattern to access the memory buffer which interacts with the file."},
    {"file-pattern", ID_FILE_PATTERN, "MODE", 0, "Pattern to access the benchmarked file."},

    /* Other Benchmark Parameters */
    {0, 0, 0, 0, "Other Parameters:", GROUP_OTHER_PARAMETERS},
    {"file", ID_FILEPATH, "PATH", OPTION_ARG_OPTIONAL, "Path to directory/file from where to benchmark.\nDefault: /tmp/"},
    {"repeats", ID_REPEATS, "N", 0, "Number of repititions for selected I/O operation."},
    /* Besides actual BYTES-Numbers we also allow KiB, MiB... */
    {"mem-buf", ID_MEM_SIZE, "BYTES", 0, "Size of the memory buffer to read from/write to."},
    {"file-buf", ID_FILE_SIZE, "BYTES", 0, "Size of the file to read from/write to."},
    {"access-size", ID_ACCESS_SIZE, "BYTES", 0, "Size of a single I/O operation."},

    /* Different Toggles */
    {0, 0, 0, 0, "Mutually exclusive Toggles:", GROUP_TOGGLES},
    {"free-ram", ID_RESTRICT_FREE_RAM, "BYTES", OPTION_ARG_OPTIONAL, "Restrict the RAM to n available byte to force cache eviction."},
    {"o-direct", ID_USE_O_DIRECT, 0, OPTION_ARG_OPTIONAL, "Use O_DIRECT to bypass the VFS page cache"},
    {"drop-cache", ID_DROP_CACHE, 0, OPTION_ARG_OPTIONAL, "Before doing anything, ask Linux to drop the page cache"},
    {"reread", ID_DO_REREAD, 0, OPTION_ARG_OPTIONAL, "Read every block twice, just benchmark the second time."},
    {"delete-afterwards", ID_DELETE_AFTERWARDS, 0, OPTION_ARG_OPTIONAL, "Delete benchmarking file afterwards"},

    /* -1 is the default group. Let's give it a name. */
    {0, 0, 0, 0, "Miscellaneous:", -1},
    {NULL}};

static struct argp argp = {
    options,
    parse_opt,
    "--read|--write --file-pattern=MODE --mem-pattern=MODE --file=[PATH] --repeats=N --mem-buf=BYTES "
    "--file-buf=BYTES --access-size=BYTES [--free-ram=BYTES] [--o-direct] [--drop-cache] [--reread] [--delete-afterwards]",
    "Note: read or write are mutually exclusive\v"
    "The file given will be overwritten, be careful when specifying it."};

void parse_benchmark_arguments(int argc, char **argv, benchmark_config_t *out)
{
  benchmark_config_parser_t parser = {
      /* Default Initialization */
      .filepath = (char *)DEFAULT_FILEPATH,
      .prepare_file_size = true,
      .read_was_selected = false,
      .write_was_selected = false,
      .free_ram_if_selected = 0,
      .o_direct_was_selected = false,
      .drop_cache_was_selected = false,
      .reread_was_selected = false,
      .delete_afterwards_was_selected = false,

      /* |{pattern1,pattern2,repeats,mem_buf,file_buf,accss_size}| = 6 */
      .number_of_missing_arugments = 6,
  };
  argp_parse(&argp, argc, argv, 0, 0, &parser);

  /* Extract config values */
  benchmark_config_t config = {
      .filepath = parser.filepath,
      .memory_buffer_in_bytes = parser.memory_buffer_in_bytes,
      .file_size_in_bytes = parser.file_size_in_bytes,
      .access_size_in_bytes = parser.access_size_in_bytes,
      .number_of_io_op_tests = parser.number_of_io_op_tests,
      .access_pattern_in_memory = parser.access_pattern_in_memory,
      .access_pattern_in_file = parser.access_pattern_in_file,
      .is_read_operation = parser.is_read_operation,
      .prepare_file_size = parser.prepare_file_size,
      .use_o_direct = parser.o_direct_was_selected,
      .restrict_free_ram_to = parser.free_ram_if_selected,
      .drop_cache_first = parser.drop_cache_was_selected,
      .do_reread = parser.reread_was_selected,
      .delete_afterwards = parser.delete_afterwards_was_selected,
  };
  memcpy(out, &config, sizeof(benchmark_config_t));
}

#pragma GCC diagnostic pop
