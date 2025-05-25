// Simple example of an AI agent interacting with the MCP server

use serde_json::json;
use std::error::Error;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the MCP server
    let url = Url::parse("ws://localhost:3000/api/ws")?;
    println!("Connecting to MCP server at {}", url);

    let (mut ws_stream, _) = connect_async(url).await?;
    println!("Connected to MCP server!");

    // Wait for the welcome message
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("Received welcome message: {}", text);
        }
    }

    // Send a request to list all components
    let list_components_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "component.list",
        "params": {}
    });
    
    ws_stream.send(Message::Text(list_components_request.to_string())).await?;
    println!("Sent component list request");

    // Wait for the response
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("Received component list response: {}", text);
        }
    }

    // Send a request to create a new component
    let create_component_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "component.create",
        "params": {
            "name": "TodoItem",
            "props": [
                {
                    "name": "title",
                    "type": "String",
                    "required": true
                },
                {
                    "name": "completed",
                    "type": "bool",
                    "required": false,
                    "default": false
                }
            ],
            "events": [
                {
                    "name": "toggle",
                    "type": "click"
                }
            ]
        }
    });
    
    ws_stream.send(Message::Text(create_component_request.to_string())).await?;
    println!("Sent component creation request");

    // Wait for the response
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("Received component creation response: {}", text);
        }
    }

    // Send a request for project information
    let project_info_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "project.info",
        "params": {}
    });
    
    ws_stream.send(Message::Text(project_info_request.to_string())).await?;
    println!("Sent project info request");

    // Wait for the response
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("Received project info response: {}", text);
        }
    }

    // Close the connection
    ws_stream.close(None).await?;
    println!("Closed connection to MCP server");

    Ok(())
}
