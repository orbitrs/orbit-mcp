// Project-related API models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Project name
    pub name: String,
    /// Project path
    pub path: PathBuf,
    /// Project version
    pub version: String,
    /// Project description
    pub description: Option<String>,
    /// Project dependencies
    pub dependencies: Vec<Dependency>,
    /// Project structure
    pub structure: ProjectStructure,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    /// Dependency version
    pub version: String,
    /// Whether this is a development dependency
    pub is_dev: bool,
}

/// Project structure information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    /// Project components
    pub components: Vec<String>,
    /// Project assets
    pub assets: Vec<String>,
    /// Project tests
    pub tests: Vec<String>,
}

/// Create project request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    /// Project name
    pub name: String,
    /// Project template
    pub template: String,
    /// Project path (optional, default is current directory)
    pub path: Option<PathBuf>,
    /// Project description
    pub description: Option<String>,
    /// Additional project dependencies
    pub dependencies: Option<Vec<Dependency>>,
}

/// Update project request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    /// Project name
    pub name: Option<String>,
    /// Project description
    pub description: Option<String>,
    /// Project version
    pub version: Option<String>,
    /// Dependencies to add
    pub add_dependencies: Option<Vec<Dependency>>,
    /// Dependencies to remove
    pub remove_dependencies: Option<Vec<String>>,
}

/// Project analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    /// Number of components
    pub component_count: usize,
    /// Number of tests
    pub test_count: usize,
    /// Test coverage percentage
    pub test_coverage: f32,
    /// Build time in milliseconds
    pub build_time_ms: u64,
    /// Issues found in the project
    pub issues: Vec<ProjectIssue>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Issue found in a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIssue {
    /// Issue code
    pub code: String,
    /// Issue message
    pub message: String,
    /// Issue location
    pub location: Option<ProjectLocation>,
    /// Issue severity
    pub severity: IssueSeverity,
}

/// Location in a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLocation {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: Option<u32>,
    /// Column number
    pub column: Option<u32>,
}

/// Issue severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Error severity
    #[serde(rename = "error")]
    Error,
    /// Warning severity
    #[serde(rename = "warning")]
    Warning,
    /// Info severity
    #[serde(rename = "info")]
    Info,
}
