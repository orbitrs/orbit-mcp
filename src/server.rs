// MCP server implementation

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::{broadcast, RwLock};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::handlers;

/// Model Context Protocol server for the Orbit Framework
pub struct McpServer {
    /// The host address
    host: String,
    /// The port number
    port: u16,
    /// The project directory
    project_dir: PathBuf,
    /// Shared server state
    state: Arc<RwLock<ServerState>>,
    /// Broadcast channel for events
    event_tx: broadcast::Sender<ServerEvent>,
}

/// Shared server state
pub struct ServerState {
    /// Number of active connections
    active_connections: usize,
    /// Connected clients
    clients: Vec<ClientInfo>,
}

/// Information about a connected client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Client ID
    pub id: String,
    /// Client name
    pub name: Option<String>,
    /// Client capabilities
    pub capabilities: Vec<String>,
    /// Connection time
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

/// Server event for broadcasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEvent {
    /// Event type
    pub event_type: String,
    /// Event payload
    pub payload: Value,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl McpServer {
    /// Create a new MCP server
    pub async fn new<P: AsRef<Path>>(host: String, port: u16, project_dir: P) -> Result<Self> {
        let (event_tx, _) = broadcast::channel(100);
        let state = Arc::new(RwLock::new(ServerState {
            active_connections: 0,
            clients: Vec::new(),
        }));

        Ok(Self {
            host,
            port,
            project_dir: project_dir.as_ref().to_path_buf(),
            state,
            event_tx,
        })
    }

    /// Run the MCP server
    pub async fn run(&self) -> Result<()> {
        let state = self.state.clone();
        let event_tx = self.event_tx.clone();
        let project_dir = self.project_dir.clone();

        // Create the API router
        let api_router = Router::new()
            // WebSocket handler
            .route("/ws", get(Self::websocket_handler))
            // Status endpoint
            .route("/status", get(handlers::status_handler))
            // Documentation endpoint
            .route("/docs", get(handlers::docs_handler))
            // Component API endpoints
            .route(
                "/components",
                get(handlers::components::list_components)
                    .post(handlers::components::create_component),
            )
            .route(
                "/components/:id",
                get(handlers::components::get_component)
                    .put(handlers::components::update_component)
                    .delete(handlers::components::delete_component),
            )
            // Project API endpoints
            .route("/project", get(handlers::projects::get_project_info))
            .route("/project/build", post(handlers::build::build_project))
            .route("/project/test", post(handlers::testing::run_tests));

        // Create the main router
        let app = Router::new()
            .nest("/api", api_router)
            .layer(Extension(state))
            .layer(Extension(event_tx))
            .layer(Extension(project_dir))
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive());

        // Start the server
        let addr = format!("{}:{}", self.host, self.port)
            .parse::<SocketAddr>()
            .context("Invalid address format")?;

        info!("MCP server listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .context("Failed to start server")?;

        Ok(())
    }

    // WebSocket handler for real-time communication
    async fn websocket_handler(
        ws: WebSocketUpgrade,
        Extension(state): Extension<Arc<RwLock<ServerState>>>,
        Extension(event_tx): Extension<broadcast::Sender<ServerEvent>>,
        Extension(project_dir): Extension<PathBuf>,
    ) -> impl IntoResponse {
        ws.on_upgrade(|socket| async {
            Self::handle_websocket_connection(socket, state, event_tx, project_dir).await;
        })
    }

    // Handle an individual WebSocket connection
    async fn handle_websocket_connection(
        socket: WebSocket,
        state: Arc<RwLock<ServerState>>,
        event_tx: broadcast::Sender<ServerEvent>,
        project_dir: PathBuf,
    ) {
        // Split the socket
        let (mut sender, mut receiver) = socket.split();

        // Generate a unique client ID
        let client_id = uuid::Uuid::new_v4().to_string();

        // Add the client to the state
        {
            let mut state = state.write().await;
            state.active_connections += 1;
            state.clients.push(ClientInfo {
                id: client_id.clone(),
                name: None,
                capabilities: Vec::new(),
                connected_at: chrono::Utc::now(),
            });
        }

        info!("New WebSocket client connected: {}", client_id);

        // Subscribe to events
        let mut event_rx = event_tx.subscribe();

        // Send welcome message
        let welcome = json!({
            "jsonrpc": "2.0",
            "method": "server.welcome",
            "params": {
                "serverId": "orbit-mcp-server",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": [
                    "components",
                    "projects",
                    "building",
                    "testing",
                    "documentation"
                ]
            }
        });

        if let Err(e) = sender.send(Message::Text(welcome.to_string())).await {
            error!("Failed to send welcome message: {}", e);
        }

        // Handle incoming messages (in a separate task)
        let state_clone = state.clone();
        let event_tx_clone = event_tx.clone();
        let project_dir_clone = project_dir.clone();
        let client_id_clone = client_id.clone();

        let receive_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(text) => {
                        // Handle JSON-RPC request
                        Self::handle_jsonrpc_message(
                            text,
                            &client_id_clone,
                            &state_clone,
                            &event_tx_clone,
                            &project_dir_clone,
                        )
                        .await;
                    }
                    Message::Close(_) => break,
                    _ => {} // Ignore other message types
                }
            }
        });

        // Forward broadcast events to the client
        let state_clone = state.clone();
        let client_id_clone = client_id.clone();

        let send_task = tokio::spawn(async move {
            while let Ok(event) = event_rx.recv().await {
                // Serialize the event to JSON
                let event_json = json!({
                    "jsonrpc": "2.0",
                    "method": "server.event",
                    "params": {
                        "type": event.event_type,
                        "payload": event.payload,
                        "timestamp": event.timestamp
                    }
                });

                // Send the event to the client
                if let Err(e) = sender.send(Message::Text(event_json.to_string())).await {
                    error!("Failed to send event to client {}: {}", client_id_clone, e);
                    break;
                }
            }
        });

        // Wait for either task to finish
        tokio::select! {
            _ = receive_task => {},
            _ = send_task => {},
        }

        // Client disconnected, update state
        {
            let mut state = state.write().await;
            state.active_connections -= 1;
            state.clients.retain(|c| c.id != client_id);
        }

        info!("WebSocket client disconnected: {}", client_id);
    }

    // Handle JSON-RPC messages
    async fn handle_jsonrpc_message(
        text: String,
        client_id: &str,
        state: &Arc<RwLock<ServerState>>,
        event_tx: &broadcast::Sender<ServerEvent>,
        project_dir: &PathBuf,
    ) {
        // Parse the JSON-RPC message
        let request: Result<Value, _> = serde_json::from_str(&text);

        match request {
            Ok(request) => {
                debug!(
                    "Received JSON-RPC request from client {}: {:?}",
                    client_id, request
                );

                // Extract method and params
                let method = request["method"].as_str();
                let id = request["id"].as_u64();

                if let Some(method) = method {
                    // Handle different methods
                    match method {
                        // Add method handling here
                        _ => {
                            debug!("Unhandled method: {}", method);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to parse JSON-RPC message: {}", e);
            }
        }
    }
}
