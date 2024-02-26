#!/bin/bash
#SBATCH -t 1:00:00
#SBATCH -p medium
#SBATCH --output=scc-%j.out
#SBATCH --mem=5G

CLUSTER="scc"

SCRIPT_DIR="/scratch/users/$(whoami)/blackheap"
cd $SCRIPT_DIR

PATH_TO_SAVE_OUTPUT="${SCRIPT_DIR}/output_${CLUSTER}"
PATH_TO_BENCHMARK_FILE="${SCRIPT_DIR}/benchmark_${CLUSTER}.dat"

./target/release/blackheap ${PATH_TO_SAVE_OUTPUT} -f ${PATH_TO_BENCHMARK_FILE}

