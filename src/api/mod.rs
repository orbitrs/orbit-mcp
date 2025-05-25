// API module definition

pub mod build;
pub mod components;
pub mod documentation;
pub mod projects;
pub mod testing;

// Common API types
use serde::{Deserialize, Serialize};
use serde_json::Value;
