#include <stdio.h>
#include <argp.h>
#include <stdbool.h>
#include<stdlib.h>

#include "io_benchmark.h"
#include "arg_parser.h"
#include "output.h"

int main(int argc, char **argv)
{
  benchmark_config_t config;
  benchmark_results_t *results;
  parse_benchmark_arguments(argc, argv, &config);

  results = benchmark_file(&config);

  print_output(&config, results);

  free(results);
  return 0;
}
