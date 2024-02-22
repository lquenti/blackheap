fn main() {
    cc::Build::new()
        .file("src/c_code/benchmarker_internal.c")
        .compile("c_benchmarker");
}
