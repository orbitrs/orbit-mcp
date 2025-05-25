// Documentation handler implementations

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

use crate::api::documentation::{DocCategory, DocItem, DocQueryRequest};
use crate::server::ServerState;
