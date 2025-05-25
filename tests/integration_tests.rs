// MCP server integration tests

#[cfg(test)]
mod tests {
    use orbit_mcp::server::McpServer;
    use serde_json::json;
    use std::path::Path;
    use std::time::Duration;
    use tokio::time::timeout;
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
    use url::Url;

    // Start the server on a dynamic port for testing
    async fn start_test_server() -> (u16, tokio::task::JoinHandle<()>) {
        // Choose a port that's likely to be free
        let port = 43210;
        let server = McpServer::new("127.0.0.1".to_string(), port, Path::new("."))
            .await
            .unwrap();

        // Start the server in a background task
        let handle = tokio::spawn(async move {
            server.run().await.unwrap();
        });

        // Give the server time to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        (port, handle)
    }

    #[tokio::test]
    async fn test_server_connection() -> Result<(), Box<dyn std::error::Error>> {
        // Start the server
        let (port, handle) = start_test_server().await;

        // Connect to the server
        let url = Url::parse(&format!("ws://127.0.0.1:{}/api/ws", port))?;
        let (mut ws_stream, _) = connect_async(url).await?;

        // Wait for the welcome message
        let msg = timeout(Duration::from_secs(1), ws_stream.next()).await??;
        if let Message::Text(text) = msg.unwrap() {
            let json: serde_json::Value = serde_json::from_str(&text)?;
            assert_eq!(json["method"], "server.welcome");
        } else {
            panic!("Expected text message");
        }

        // Clean up
        ws_stream.close(None).await?;
        handle.abort();

        Ok(())
    }

    #[tokio::test]
    async fn test_status_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        // Start the server
        let (port, handle) = start_test_server().await;

        // Call the status endpoint
        let response = reqwest::get(format!("http://127.0.0.1:{}/api/status", port)).await?;

        assert_eq!(response.status(), 200);

        let json: serde_json::Value = response.json().await?;
        assert_eq!(json["status"], "running");

        // Clean up
        handle.abort();

        Ok(())
    }
}
