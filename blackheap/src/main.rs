use crate::{assets::progress::Operation, cli::Cli};

use benchmark::Benchmark;
use blackheap_benchmarker::ErrorCodes;
use clap::Parser;
use tracing::{error, info};

mod assets;
mod benchmark;
mod cli;

fn main() {
    /* Init boilerplate */
    human_panic::setup_panic!();
    tracing_subscriber::fmt::init();

    /* CLI parsing */
    info!("Parsing and validating CLI");
    let cli = Cli::parse();
    if let Err(e) = cli::validate_cli(&cli) {
        error!("{:?}", e);
        std::process::exit(1);
    }

    /* Load previous results */
    info!("Trying to load previous results");
    let benchmarks = Benchmark::get_all_benchmarks(cli.drop_caches, cli.file.to_str().unwrap());
    let progress = benchmark::load_or_create_progress(&cli.to, &benchmarks);
    if let Err(e) = progress {
        error!("Error creating or parsing progress!");
        error!("{:?}", e);
        error!("Consider resetting the {:?} directory", &cli.to);
        std::process::exit(1);
    }
    let mut progress = progress.unwrap();

    /* The actual benchmarking */
    for b in benchmarks.iter() {
        /* Which access sizes do we still have to do? */
        let missing_access_sizes = {
            /* To make the borrow checker happy */
            let tmp_progress = progress.clone();
            tmp_progress
                .get_missing_access_sizes(b)
                .map(|slice| slice.to_vec())
        };
        if missing_access_sizes.is_none() {
            info!(
                "Benchmark {:?} ({:?}) already computed",
                &b.scenario,
                Operation::from_is_read_op(b.config.is_read_operation)
            );
            continue;
        }
        let missing_access_sizes: Vec<u32> = missing_access_sizes.unwrap();
        info!(
            "Benchmark {:?} ({:?}): Missing Access Sizes: {:?}",
            &b.scenario,
            Operation::from_is_read_op(b.config.is_read_operation),
            &missing_access_sizes
        );

        /* Do a benchmark for each access size */
        for access_size in missing_access_sizes {
            /* Set the access size */
            let mut config = b.config.clone();
            config.access_size_in_bytes = access_size as usize;

            /* Run the benchmark */
            info!(
                "Running {:?} ({:?}): Access Size: {:?}",
                &b.scenario,
                Operation::from_is_read_op(b.config.is_read_operation),
                access_size
            );
            let results = blackheap_benchmarker::benchmark_file(&config);
            if results.res != ErrorCodes::Success {
                info!(
                    "Error {:?} ({:?}): Access Size: {:?} failed with {:?}",
                    &b.scenario,
                    Operation::from_is_read_op(b.config.is_read_operation),
                    access_size,
                    &results.res
                );
            }

            /* Save the result; update and save the progress struct */
            info!("Saving the results");
            let res =
                benchmark::save_and_update_progress(b, access_size, &results, &cli, &mut progress);
            if let Err(e) = res {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
    }

    /* Dump all assets for Analysis */
    info!("Saving all assets info {:?}", cli.to);
    let res = assets::dump_assets(&cli.to);
    if let Err(e) = res {
        error!("{:?}", e);
        std::process::exit(1);
    }

    /* Create a CSV with all outputs we have
     *
     * Note that we can't do this while we do the single benchmarks
     * becase this would break our benchmark resume approach.
     * There, the strategy is whenever a folder exists but the benchmark
     * is not yet completely finished, it got killed using the write.
     * As a solution, we delete the full folder and benchmark that access size again.
     *
     * This is not possible here; if we delete the full csv we are back to square one.
     */
    info!("Creating a csv of all results");
    let res = benchmark::create_csv_of_all_measurements(&cli.to);
    if let Err(e) = res {
        error!("{:?}", e);
        std::process::exit(1);
    }

    /* Print out how to use the assets, refer to the README */
    info!("Benchmark ran successfully! See the README for how to run the automated, Python-based analysis.");
}
