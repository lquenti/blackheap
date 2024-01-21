fn main() {
    cc::Build::new()
        .file("src/c_code/benchmarker.c")
        .compile("c_benchmarker");
}
