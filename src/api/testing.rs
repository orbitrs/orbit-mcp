// Testing-related API models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Test options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOptions {
    /// Test filter
    pub filter: Option<String>,
    /// Whether to run with test coverage
    pub coverage: bool,
    /// Whether to run with verbose output
    pub verbose: bool,
    /// Whether to continue on error
    pub no_fail_fast: bool,
}

/// Test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Number of tests run
    pub tests_run: u32,
    /// Number of tests passed
    pub tests_passed: u32,
    /// Number of tests failed
    pub tests_failed: u32,
    /// Number of tests ignored
    pub tests_ignored: u32,
    /// Total time taken
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    /// Test failures
    pub failures: Vec<TestFailure>,
    /// Test coverage data
    pub coverage: Option<TestCoverage>,
}

/// Test failure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailure {
    /// Test name
    pub test_name: String,
    /// Failure message
    pub message: String,
    /// Stack trace
    pub stack_trace: String,
    /// Failure location
    pub location: TestLocation,
}

/// Test location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestLocation {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
}

/// Test coverage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCoverage {
    /// Line coverage percentage
    pub line_coverage: f32,
    /// Branch coverage percentage
    pub branch_coverage: f32,
    /// Function coverage percentage
    pub function_coverage: f32,
    /// Coverage by file
    pub files: Vec<FileCoverage>,
}

/// File coverage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    /// File path
    pub file: PathBuf,
    /// Line coverage percentage
    pub line_coverage: f32,
    /// Number of covered lines
    pub lines_covered: u32,
    /// Number of total lines
    pub lines_total: u32,
}

/// Test request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRequest {
    /// Test options
    pub options: TestOptions,
}
