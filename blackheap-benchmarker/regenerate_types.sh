#!/bin/bash

# This script generates Rust types for a benchmarker from a C header file using Bindgen.

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if ! command -v bindgen &> /dev/null; then
    echo "Error: Bindgen is not found in your PATH."
    echo "Please install bindgen by running: \$ cargo install bindgen-cli"
    exit 1
fi

bindgen ${SCRIPT_DIR}/src/c_code/benchmarker.h -o ${SCRIPT_DIR}/src/c_code/benchmarker.rs

if [ $? -eq 0 ]; then
    echo "Bindgen completed successfully. Rust types generated in src/c_code/benchmarker.rs."
else
    echo "Error: Bindgen encountered an issue while generating Rust types."
    exit 1
fi
