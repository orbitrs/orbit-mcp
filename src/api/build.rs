// Build-related API models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Build options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildOptions {
    /// Build target
    pub target: BuildTarget,
    /// Build mode
    pub mode: BuildMode,
    /// Whether to enable optimization
    pub optimize: bool,
    /// Whether to generate source maps
    pub source_maps: bool,
    /// Additional features to enable
    pub features: Vec<String>,
}

/// Build target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildTarget {
    /// Web target
    #[serde(rename = "web")]
    Web,
    /// Native target
    #[serde(rename = "native")]
    Native,
    /// WebAssembly target
    #[serde(rename = "wasm")]
    Wasm,
}

/// Build mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildMode {
    /// Development mode
    #[serde(rename = "development")]
    Development,
    /// Production mode
    #[serde(rename = "production")]
    Production,
}

/// Build result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    /// Whether the build was successful
    pub success: bool,
    /// Time taken in milliseconds
    pub time_ms: u64,
    /// Output files
    pub output_files: Vec<PathBuf>,
    /// Build errors
    pub errors: Vec<BuildError>,
    /// Build warnings
    pub warnings: Vec<BuildWarning>,
}

/// Build error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error location
    pub location: Option<BuildLocation>,
}

/// Build warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildWarning {
    /// Warning code
    pub code: String,
    /// Warning message
    pub message: String,
    /// Warning location
    pub location: Option<BuildLocation>,
}

/// Build location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildLocation {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
}

/// Build request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildRequest {
    /// Build options
    pub options: BuildOptions,
}
