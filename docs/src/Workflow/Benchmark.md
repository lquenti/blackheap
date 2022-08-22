# Creating Benchmarks

In order to create accurate benchmarks of specific characteristics, we created the `blackheap-benchmark` benchmarker, which is based on [`io-modelling`](https://github.com/JulianKunkel/io-modelling).

It is standalone, configurable via CLI parameters and prints any JSON output to stdout.

When using blackheap as a closed solution, the benchmarker shouldn't be used directly.

## Parameters

It supports the following parameters:

- `-r`/`-w` for either read or write operations
- `--file-pattern`/`--mem-pattern` for the access pattern of the file system or memory buffer.
  The following patterns are implemented
  - `off0`: Always use the same to the same offset
  - `seq`: Read/Write sequentially
  - `rnd`: Change the offset randomly after every I/O operation.
- `--access-size`: The size of each I/O operation. This is primarily used to create different measurements of the same characteristics.
- `--file`: On which file the benchmark should run. Note that the file at the given path will be zeroed out. Use this setting to implicitly set which file system gets benchmarked.
- `--file-buf`/`--mem-buf`: The size of the corresponding buffers in bytes. Note that the file buffer actually gets allocated via successive write operations; no fallocate is used.
- `--repeats` The number of measurements with a given configuration.
- `--delete-afterwards`: Whether the file should be deleted after the benchmark is done. This can be used to remove any page caching between benchmarks. Obviously, not deleting the file can significantly reduce the execution time of further benchmarks.
- `--drop-cache`: Drops the page cache [via `/proc/sys/vm/drop_caches`](https://www.kernel.org/doc/Documentation/sysctl/vm.txt)
- `--free-ram` Aritificially allocates RAM until the number of bytes are met. This restriction is done in order to force cache eviction.
- `--o-direct`: Uses Linux Direct I/O (see `O_DIRECT`)
- `--reread`: Read each block twice, just benchmark the second time. This is done to get cache speeds.
