// Component handler implementations

use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::components::{Component, CreateComponentRequest, UpdateComponentRequest};
use crate::server::ServerState;

// List all components
pub async fn list_components(Extension(project_dir): Extension<PathBuf>) -> impl IntoResponse {
    // TODO: Implement component listing logic
    // For now, return a mock response
    let components = vec![
        Component {
            id: "counter".to_string(),
            name: "Counter".to_string(),
            path: project_dir.join("src/counter.orbit"),
            props: vec![],
            children: false,
            events: vec![],
        },
        Component {
            id: "user-profile".to_string(),
            name: "UserProfile".to_string(),
            path: project_dir.join("src/user-profile.orbit"),
            props: vec![],
            children: false,
            events: vec![],
        },
    ];

    (StatusCode::OK, Json(json!({ "components": components })))
}

// Create a new component
pub async fn create_component(
    Extension(project_dir): Extension<PathBuf>,
    Json(request): Json<CreateComponentRequest>,
) -> impl IntoResponse {
    // TODO: Implement component creation logic
    // For now, return a mock response
    let component = Component {
        id: request.name.to_lowercase(),
        name: request.name,
        path: project_dir.join(format!("src/{}.orbit", request.name.to_lowercase())),
        props: request.props.unwrap_or_default(),
        children: request.has_children.unwrap_or(false),
        events: request.events.unwrap_or_default(),
    };

    (StatusCode::CREATED, Json(json!({ "component": component })))
}

// Get component by ID
pub async fn get_component(
    Extension(project_dir): Extension<PathBuf>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // TODO: Implement component retrieval logic
    // For now, return a mock response
    let component = Component {
        id: id.clone(),
        name: id
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>()
            + &id[1..],
        path: project_dir.join(format!("src/{}.orbit", id)),
        props: vec![],
        children: false,
        events: vec![],
    };

    (StatusCode::OK, Json(json!({ "component": component })))
}

// Update component by ID
pub async fn update_component(
    Extension(project_dir): Extension<PathBuf>,
    Path(id): Path<String>,
    Json(request): Json<UpdateComponentRequest>,
) -> impl IntoResponse {
    // TODO: Implement component update logic
    // For now, return a mock response
    let component = Component {
        id: id.clone(),
        name: id
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>()
            + &id[1..],
        path: project_dir.join(format!("src/{}.orbit", id)),
        props: request.props.unwrap_or_default(),
        children: false,
        events: request.events.unwrap_or_default(),
    };

    (StatusCode::OK, Json(json!({ "component": component })))
}

// Delete component by ID
pub async fn delete_component(
    Extension(project_dir): Extension<PathBuf>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // TODO: Implement component deletion logic
    // For now, return a mock response

    (
        StatusCode::OK,
        Json(json!({ "success": true, "message": format!("Component {} deleted", id) })),
    )
}
