// API module definition

pub mod components;
pub mod projects;
pub mod build;
pub mod testing;
pub mod documentation;

// Common API types
use serde::{Deserialize, Serialize};
use serde_json::Value;
