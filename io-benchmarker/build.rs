use std::env;
use std::path::PathBuf;

fn main() {
    let src = [
        "./src/arg_parser.c",
        "./src/helper.c",
        "./src/io_benchmark.c",
        "./src/output.c"
    ];

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Wpedantic")
        .flag("-std=gnu11")
        .flag("-O2")
        .flag("-D_GNU_SOURCE")
        .flag("-lrt");
    build.compile("io_benchmark");

    let bindings = bindgen::Builder::default()
        .header("./src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
