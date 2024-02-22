use std::{path::{PathBuf, Path}, io, fs::{self, File}};

use clap::{Parser, ValueEnum};
use thiserror::Error;
use uuid::Uuid;


#[derive(Error, Debug)]
pub enum CliError {
    #[error("Roor privileges are required for dropping caches")]
    RootRequired,

    #[error("Directory \"{0}\" does not exist and could not be created")]
    CannotCreateDirectory(PathBuf),

    #[error("Path \"{0}\" is not a directory")]
    NotADirectory(PathBuf),

    #[error("Directory \"{0}\" is not writable")]
    DirectoryNotWritable(PathBuf),

    #[error("File \"{0}\" already exists and can not be deleted")]
    CannotDeleteFile(PathBuf),

    #[error("File \"{0}\" could not be created")]
    CannotCreateFile(PathBuf),

    #[error("{0}")]
    IoError(#[from] io::Error),
}

fn is_root() -> bool {
    let euid = unsafe { libc::geteuid() };
    euid == 0
}

fn validate_output_directory(dir: &Path) -> Result<(), CliError> {
    /*
    The output directory is invalid if one of the following is true
      - the path does not exist and cant be created
      - the path exists but is not a directory
      - the path exists, is a directory but not writable
    */
    
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|_| CliError::CannotCreateDirectory(dir.to_path_buf()))?;
    }

    if !dir.is_dir() {
        return Err(CliError::NotADirectory(dir.to_path_buf()));
    }

    /* use a unique name to not destroy anything */
    let test_file = {
        let mut test_file;
        loop {
            test_file = dir.join(Uuid::new_v4().to_string());
            if !test_file.exists() {
                break;
            }
        }
        test_file
    };
    File::create(&test_file).and_then(|_| fs::remove_file(&test_file))
        .map_err(|_| CliError::DirectoryNotWritable(dir.to_path_buf()))?;

    Ok(())
}

fn validate_benchmark_file(file: &Path) -> Result<(), CliError> {
    /*
    The benchmark file is invalid if 
    - it doesnt exist and cannot be created
    - it cannot be removed
    */
    if !file.exists() {
        File::create(file).map_err(|_| CliError::CannotCreateFile(file.to_path_buf()))?;
    }

    fs::remove_file(file).map_err(|_| CliError::CannotDeleteFile(file.to_path_buf()))?;

    Ok(())
}

pub fn validate_cli(cli: &Cli) -> Result<(), CliError> {
    if cli.drop_caches && !is_root() {
        return Err(CliError::RootRequired);
    }

    validate_output_directory(&cli.to)?;
    validate_benchmark_file(&cli.file)?;

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Model {
    Linear,
    ConstantLinear,
}

/// A blackbox modeller for I/O-classification
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Output directory for all the benchmarks and results.
    /// Also used to store progress.
    to: PathBuf,

    /// Path to where the benchmark should be done
    #[clap(short, long, default_value = "/tmp/blackheap_benchmark_test_file.dat")]
    file: PathBuf,

    /// Which PredictionModel to use
    #[clap(short, long, value_enum, default_value_t = Model::ConstantLinear)]
    model: Model,

    /// Drop caches (requires root)
    #[clap(long)]
    drop_caches: bool,

}

