// Main entry point for the MCP server

use anyhow::Result;
use clap::Parser;
use log::info;
use orbit_mcp::server::McpServer;

/// Model Context Protocol (MCP) server for the Orbit Framework
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Port to use for the MCP server
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Host address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    /// Path to the Orbit project
    #[arg(short, long)]
    project: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let args = CliArgs::parse();

    // Determine the project directory
    let project_dir = args
        .project
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    // Create and start the MCP server
    info!(
        "Starting Orbit MCP server at {}:{} for project at {:?}",
        args.host, args.port, project_dir
    );

    let server = McpServer::new(args.host, args.port, project_dir).await?;
    server.run().await?;

    Ok(())
}
