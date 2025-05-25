// Component-related API models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Component information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Component ID
    pub id: String,
    /// Component name
    pub name: String,
    /// Component path
    pub path: PathBuf,
    /// Component properties
    pub props: Vec<ComponentProp>,
    /// Component children
    pub children: bool,
    /// Component events
    pub events: Vec<ComponentEvent>,
}

/// Component property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProp {
    /// Property name
    pub name: String,
    /// Property type
    pub r#type: String,
    /// Whether the property is required
    pub required: bool,
    /// Property default value (if any)
    pub default: Option<serde_json::Value>,
}

/// Component event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentEvent {
    /// Event name
    pub name: String,
    /// Event type
    pub r#type: String,
}

/// Create component request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateComponentRequest {
    /// Component name
    pub name: String,
    /// Component path (relative to project root)
    pub path: Option<String>,
    /// Component properties
    pub props: Option<Vec<ComponentProp>>,
    /// Whether the component has children
    pub has_children: Option<bool>,
    /// Component events
    pub events: Option<Vec<ComponentEvent>>,
    /// Component template (if using a template)
    pub template: Option<String>,
}

/// Update component request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateComponentRequest {
    /// Component properties to update
    pub props: Option<Vec<ComponentProp>>,
    /// Component events to update
    pub events: Option<Vec<ComponentEvent>>,
    /// Component code to update
    pub code: Option<String>,
    /// Component template to update
    pub template: Option<String>,
    /// Component style to update
    pub style: Option<String>,
}

/// Component analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentAnalysis {
    /// Component complexity score (0-100)
    pub complexity: u8,
    /// Component dependencies
    pub dependencies: Vec<String>,
    /// Issues found in the component
    pub issues: Vec<ComponentIssue>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Issue found in a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentIssue {
    /// Issue code
    pub code: String,
    /// Issue message
    pub message: String,
    /// Issue location
    pub location: ComponentLocation,
    /// Issue severity
    pub severity: IssueSeverity,
}

/// Location in a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLocation {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
}

/// Issue severity
#[derive(Debug, Clone, Serialize, Deserialize)]
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
