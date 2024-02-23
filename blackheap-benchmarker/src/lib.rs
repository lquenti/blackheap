mod c_code;

use c_code::benchmarker as b;

use libc::c_char;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub enum AccessPattern {
    Const,
    Sequential,
    Random,
}

impl AccessPattern {
    pub fn to_c_code(&self) -> b::access_pattern {
        match self {
            Self::Const => b::access_pattern_ACCESS_PATTERN_CONST,
            Self::Sequential => b::access_pattern_ACCESS_PATTERN_SEQUENTIAL,
            Self::Random => b::access_pattern_ACCESS_PATTERN_RANDOM,
        }
    }

    pub fn from_c_code(n: b::access_pattern) -> Self {
        match n {
            b::access_pattern_ACCESS_PATTERN_CONST => Self::Const,
            b::access_pattern_ACCESS_PATTERN_SEQUENTIAL => Self::Sequential,
            b::access_pattern_ACCESS_PATTERN_RANDOM => Self::Random,
            _ => {
                panic!("Unknown Access Pattern! Probably forgot to update Rust to C logic");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCodes {
    Success,

    /* Linux operations that failed */
    MallocFailed,
    OpenFailed,
    ReadFailed,
    WriteFailed,
    LseekFailed,
    FsyncFailed,
    FstatFailed,
    IOOpFailed,
    RemoveFailed,

    /* High Level Operations */
    DropPageCacheFailedNoPermissions,
    DropPageCacheFailedOther,
    IncorrectFileBufferSize,
}

impl ErrorCodes {
    pub fn to_c_code(&self) -> b::error_codes {
        match self {
            Self::Success => b::error_codes_ERROR_CODES_SUCCESS,
            Self::MallocFailed => b::error_codes_ERROR_CODES_MALLOC_FAILED,
            Self::OpenFailed => b::error_codes_ERROR_CODES_OPEN_FAILED,
            Self::ReadFailed => b::error_codes_ERROR_CODES_READ_FAILED,
            Self::WriteFailed => b::error_codes_ERROR_CODES_WRITE_FAILED,
            Self::LseekFailed => b::error_codes_ERROR_CODES_LSEEK_FAILED,
            Self::FsyncFailed => b::error_codes_ERROR_CODES_FSYNC_FAILED,
            Self::FstatFailed => b::error_codes_ERROR_CODES_FSTAT_FAILED,
            Self::IOOpFailed => b::error_codes_ERROR_CODES_IO_OP_FAILED,
            Self::RemoveFailed => b::error_codes_ERROR_CODES_REMOVE_FAILED,
            Self::DropPageCacheFailedNoPermissions => {
                b::error_codes_ERROR_CODES_DROP_PAGE_CACHE_FAILED_NO_PERMISSIONS
            }
            Self::DropPageCacheFailedOther => {
                b::error_codes_ERROR_CODES_DROP_PAGE_CACHE_FAILED_OTHER
            }
            Self::IncorrectFileBufferSize => b::error_codes_ERROR_CODES_INCORRECT_FILE_BUFFER_SIZE,
        }
    }

    pub fn from_c_code(n: b::error_codes) -> Self {
        match n {
            b::error_codes_ERROR_CODES_SUCCESS => Self::Success,
            b::error_codes_ERROR_CODES_MALLOC_FAILED => Self::MallocFailed,
            b::error_codes_ERROR_CODES_OPEN_FAILED => Self::OpenFailed,
            b::error_codes_ERROR_CODES_READ_FAILED => Self::ReadFailed,
            b::error_codes_ERROR_CODES_WRITE_FAILED => Self::WriteFailed,
            b::error_codes_ERROR_CODES_LSEEK_FAILED => Self::LseekFailed,
            b::error_codes_ERROR_CODES_FSYNC_FAILED => Self::FsyncFailed,
            b::error_codes_ERROR_CODES_FSTAT_FAILED => Self::FstatFailed,
            b::error_codes_ERROR_CODES_IO_OP_FAILED => Self::IOOpFailed,
            b::error_codes_ERROR_CODES_REMOVE_FAILED => Self::RemoveFailed,
            b::error_codes_ERROR_CODES_DROP_PAGE_CACHE_FAILED_NO_PERMISSIONS => {
                Self::DropPageCacheFailedNoPermissions
            }
            b::error_codes_ERROR_CODES_DROP_PAGE_CACHE_FAILED_OTHER => {
                Self::DropPageCacheFailedOther
            }
            b::error_codes_ERROR_CODES_INCORRECT_FILE_BUFFER_SIZE => Self::IncorrectFileBufferSize,
            _ => panic!("Unknown Error Code! Probably forgot to update Rust to C logic"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub filepath: String,
    pub memory_buffer_in_bytes: usize,
    pub file_size_in_bytes: usize,
    pub access_size_in_bytes: usize,
    pub number_of_io_op_tests: usize,
    pub access_pattern_in_memory: AccessPattern,
    pub access_pattern_in_file: AccessPattern,
    pub is_read_operation: bool,
    pub prepare_file_size: bool,
    pub drop_cache_first: bool,
    pub do_reread: bool,
    pub restrict_free_ram_to: Option<usize>,
}

impl BenchmarkConfig {
    pub fn to_c_code(&self) -> b::benchmark_config {
        let filepath_cstr = CString::new(self.filepath.clone()).expect("CString::new failed");
        b::benchmark_config {
            filepath: filepath_cstr.into_raw() as *const c_char,
            memory_buffer_in_bytes: self.memory_buffer_in_bytes,
            file_size_in_bytes: self.file_size_in_bytes,
            access_size_in_bytes: self.access_size_in_bytes,
            number_of_io_op_tests: self.number_of_io_op_tests,
            access_pattern_in_memory: self.access_pattern_in_memory.to_c_code(),
            access_pattern_in_file: self.access_pattern_in_file.to_c_code(),
            is_read_operation: self.is_read_operation,
            prepare_file_size: self.prepare_file_size,
            drop_cache_first: self.drop_cache_first,
            do_reread: self.do_reread,
            restrict_free_ram_to: self.restrict_free_ram_to.unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub res: ErrorCodes,
    pub durations: Vec<f64>,
}

impl BenchmarkResults {
    unsafe fn from_c_code(c_results: b::benchmark_results) -> Self {
        let res = ErrorCodes::from_c_code(c_results.res);

        let durations = if c_results.length > 0 && !c_results.durations.is_null() {
            std::slice::from_raw_parts(c_results.durations, c_results.length).to_vec()
        } else {
            Vec::new()
        };

        libc::free(c_results.durations as *mut libc::c_void);

        BenchmarkResults { res, durations }
    }
}

pub fn benchmark_file(config: &BenchmarkConfig) -> BenchmarkResults {
    let c_config = config.to_c_code();

    unsafe {
        let c_results = b::benchmark_file(&c_config);
        BenchmarkResults::from_c_code(c_results)
    }
}
