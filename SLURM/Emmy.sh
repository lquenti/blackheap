#!/bin/bash
#SBATCH -t 1:00:00
#SBATCH -p standard96
#SBATCH --output=emmy-%j.out
#SBATCH --exclusive
#SBATCH --partition=standard96
#SBATCH --constraint=ssd

CLUSTER="emmy"

SCRIPT_DIR="/scratch-emmy/usr/$(whoami)/blackheap"
cd $SCRIPT_DIR

PATH_TO_SAVE_OUTPUT="${SCRIPT_DIR}/output_${CLUSTER}"
PATH_TO_BENCHMARK_FILE="${SCRIPT_DIR}/benchmark_${CLUSTER}.dat"

./target/release/blackheap ${PATH_TO_SAVE_OUTPUT} -f ${PATH_TO_BENCHMARK_FILE}

rm $PATH_TO_BENCHMARK_FILE
