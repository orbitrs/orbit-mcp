// Documentation-related API models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Documentation query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocQueryRequest {
    /// Search query
    pub query: String,
    /// Documentation category filter
    pub category: Option<DocCategory>,
    /// Maximum number of results
    pub limit: Option<usize>,
}

/// Documentation category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocCategory {
    /// API documentation
    #[serde(rename = "api")]
    Api,
    /// Guide documentation
    #[serde(rename = "guide")]
    Guide,
    /// Tutorial documentation
    #[serde(rename = "tutorial")]
    Tutorial,
    /// Example documentation
    #[serde(rename = "example")]
    Example,
}

/// Documentation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocItem {
    /// Documentation title
    pub title: String,
    /// Documentation path
    pub path: PathBuf,
    /// Documentation category
    pub category: DocCategory,
    /// Short description
    pub description: String,
    /// Full content
    pub content: Option<String>,
    /// Related items
    pub related: Vec<String>,
}

/// Documentation example request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocExampleRequest {
    /// Type of example (component, project, feature)
    pub example_type: String,
    /// Specific feature or concept to demonstrate
    pub feature: Option<String>,
    /// Maximum number of examples to return
    pub limit: Option<usize>,
}

/// Documentation example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocExample {
    /// Example title
    pub title: String,
    /// Example description
    pub description: String,
    /// Example code
    pub code: String,
    /// Example language
    pub language: String,
    /// Related examples
    pub related: Vec<String>,
    /// Example path
    pub path: Option<PathBuf>,
}
