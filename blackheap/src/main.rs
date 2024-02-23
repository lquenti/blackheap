use crate::{assets::progress::Operation, cli::Cli};

use benchmark::Benchmark;
use blackheap_benchmarker::ErrorCodes;
use clap::Parser;
use tracing::{debug, error, info};

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
    debug!("{:?}", &cli);
    if let Err(e) = cli::validate_cli(&cli) {
        error!("{:?}", e);
        std::process::exit(1);
    }

    /* Load previous results */
    info!("Trying to load previous results");
    let benchmarks = Benchmark::get_all_benchmarks(cli.drop_caches, cli.file.to_str().unwrap());
    let progress = benchmark::load_or_create_progress(&cli.to, &benchmarks);
    if let Err(e) = progress {
        error!("{:?}", e);
        std::process::exit(1);
    }
    let mut progress = progress.unwrap();

    /* The actual benchmarking */
    for b in benchmarks.iter() {
        /* Which access sizes do we still have to do? */
        let missing_access_sizes = {
            let tmp_progress = progress.clone();
            tmp_progress
                .get_missing_access_sizes(&b)
                .map(|slice| slice.to_vec())
        };
        if None == missing_access_sizes {
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
                "Running {:?} ({:?}): Access Sizes: {:?}",
                &b.scenario,
                Operation::from_is_read_op(b.config.is_read_operation),
                access_size
            );
            let results = blackheap_benchmarker::benchmark_file(&config);
            if results.res != ErrorCodes::Success {
                info!(
                    "Error {:?} ({:?}): Access Sizes: {:?} failed with {:?}",
                    &b.scenario,
                    Operation::from_is_read_op(b.config.is_read_operation),
                    access_size,
                    &results.res
                );
            }

            /* Save the result; update and save the progress struct */
            info!("Saving the results");
            let res =
                benchmark::save_and_update_progress(&b, access_size, &results, &cli, &mut progress);
            if let Err(e) = res {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
    }

    /* Do the regression for all benchmarks */

    /* Save the regression (should be part of impl Model) */

    /* Dump all assets for Analysis */

    /* Print out how to use the assets, refer to the README */
}
