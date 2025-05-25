// Project handler implementations

use anyhow::{Context, Result};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::projects::{Dependency, Project, ProjectStructure};
use crate::server::ServerState;

// Get project information
pub async fn get_project_info(Extension(project_dir): Extension<PathBuf>) -> impl IntoResponse {
    // TODO: Implement project info retrieval logic
    // For now, return a mock response
    let project = Project {
        name: "orbit-example".to_string(),
        path: project_dir.clone(),
        version: "0.1.0".to_string(),
        description: Some("Orbit example project".to_string()),
        dependencies: vec![
            Dependency {
                name: "orbit".to_string(),
                version: "0.1.0".to_string(),
                is_dev: false,
            },
            Dependency {
                name: "orlint".to_string(),
                version: "0.1.0".to_string(),
                is_dev: true,
            },
        ],
        structure: ProjectStructure {
            components: vec![
                "counter.orbit".to_string(),
                "user-profile.orbit".to_string(),
            ],
            assets: vec![],
            tests: vec![],
        },
    };

    (StatusCode::OK, Json(json!({ "project": project })))
}
