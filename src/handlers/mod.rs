// Handler module definition

pub mod components;
pub mod projects;
pub mod build;
pub mod testing;
pub mod documentation;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::server::ServerState;

// Handle status requests
pub async fn status_handler(
    axum::extract::Extension(state): axum::extract::Extension<Arc<RwLock<ServerState>>>,
) -> impl IntoResponse {
    let state = state.read().await;
    let status = json!({
        "status": "running",
        "active_connections": state.active_connections,
        "clients": state.clients,
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": 0, // TODO: Track uptime
    });

    (StatusCode::OK, Json(status))
}

// Handle documentation requests
pub async fn docs_handler() -> impl IntoResponse {
    let docs = json!({
        "title": "Orbit MCP Server API",
        "description": "API documentation for the Orbit MCP Server",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "api/status": "Get server status",
            "api/docs": "Get API documentation",
            "api/ws": "WebSocket endpoint for JSON-RPC communication",
            "api/components": "Component management",
            "api/project": "Project information",
            "api/project/build": "Build project",
            "api/project/test": "Run tests"
        }
    });

    (StatusCode::OK, Json(docs))
}
