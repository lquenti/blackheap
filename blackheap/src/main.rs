use blackheap_benchmarker::{BenchmarkConfig, AccessPattern};

fn main() {
    let cfg = BenchmarkConfig {
        filepath: String::from("/tmp/test_file.bin"),
        memory_buffer_in_bytes: 1024,
        file_size_in_bytes: 1024 * 10,
        access_size_in_bytes: 128,
        number_of_io_op_tests: 10,
        access_pattern_in_memory: AccessPattern::Sequential,
        access_pattern_in_file: AccessPattern::Sequential,
        is_read_operation: true,
        prepare_file_size: true,
        drop_cache_first: false,
        do_reread: false,
        restrict_free_ram_to: None
    };
    let res = blackheap_benchmarker::benchmark_file(&cfg);
    println!("{:?}", res.res);
    for val in res.durations {
        println!("{}", val);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
}
