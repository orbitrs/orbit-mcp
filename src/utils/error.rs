// Error handling utilities

use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    
    #[error("Build error: {0}")]
    BuildError(String),
    
    #[error("Test error: {0}")]
    TestError(String),
    
    #[error("Documentation error: {0}")]
    DocumentationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Server error: {0}")]
    ServerError(String),
}

// Implement conversion from McpError to HTTP status code
impl McpError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        use axum::http::StatusCode;
        
        match self {
            McpError::ComponentNotFound(_) => StatusCode::NOT_FOUND,
            McpError::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            McpError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
