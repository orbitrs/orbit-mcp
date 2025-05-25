// Example integration with VS Code extension

use serde_json::json;
use std::error::Error;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Simulate a VS Code extension connecting to the MCP server
    println!("VS Code Extension: Connecting to MCP server...");

    // Connect to the MCP server
    let url = Url::parse("ws://localhost:3000/api/ws")?;
    let (mut ws_stream, _) = connect_async(url).await?;
    println!("VS Code Extension: Connected to MCP server!");

    // Wait for the welcome message
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("VS Code Extension: Received welcome message: {}", text);
        }
    }

    // Simulate user actions in VS Code
    println!("\nUser opens Orbit project in VS Code");

    // Send a request for project information
    let project_info_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "project.info",
        "params": {}
    });

    ws_stream
        .send(Message::Text(project_info_request.to_string()))
        .await?;

    // Wait for the response
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("VS Code Extension: Received project structure");
            println!("VS Code: Displaying project structure in Explorer view");
        }
    }

    // Simulate user requesting to create a new component via VS Code command
    println!("\nUser executes 'Create New Component' command");

    // Send a request to create a new component
    let create_component_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "component.create",
        "params": {
            "name": "NavBar",
            "template": "basic",
            "path": "src/components",
            "props": [
                {
                    "name": "title",
                    "type": "String",
                    "required": true
                },
                {
                    "name": "items",
                    "type": "Vec<MenuItem>",
                    "required": true
                }
            ],
            "has_children": true
        }
    });

    ws_stream
        .send(Message::Text(create_component_request.to_string()))
        .await?;

    // Wait for the response
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("VS Code Extension: Component created successfully");
            println!("VS Code: Opening component in editor");
        }
    }

    // Simulate user building the project
    println!("\nUser executes 'Build Project' command");

    // Send a request to build the project
    let build_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "build.compile",
        "params": {
            "options": {
                "target": "web",
                "mode": "development",
                "optimize": false,
                "source_maps": true,
                "features": []
            }
        }
    });

    ws_stream
        .send(Message::Text(build_request.to_string()))
        .await?;

    // Simulate build in progress
    println!("VS Code: Showing build progress in status bar");
    sleep(Duration::from_millis(1000)).await;

    // Wait for the response
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("VS Code Extension: Build completed");
            println!("VS Code: Displaying build results in Problems panel");
        }
    }

    // Simulate the VS Code extension handling server events
    println!("\nServer sends component change notification");

    // Wait for any server event
    if let Some(Ok(message)) = ws_stream.next().await {
        if let Message::Text(text) = message {
            println!("VS Code Extension: Received server event: {}", text);
            println!("VS Code: Updating Explorer view to reflect changes");
        }
    }

    // Simulate user closing VS Code
    println!("\nUser closes VS Code");
    ws_stream.close(None).await?;
    println!("VS Code Extension: Disconnected from MCP server");

    Ok(())
}
