# Orbit MCP Server Implementation

## Overview

The Model Context Protocol (MCP) server for the Orbit Framework exposes development APIs to AI agents, enabling them to interact with the Orbit framework for tasks like creating components, modifying code, running tests, accessing documentation, and debugging issues.

## Project Structure

```
orbit-mcp/
├── Cargo.toml              # Project manifest
├── README.md               # Project documentation
├── build.rs                # Build script
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library exports
│   ├── server.rs           # MCP server implementation
│   ├── api/                # API models
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   ├── projects.rs
│   │   ├── build.rs
│   │   ├── testing.rs
│   │   └── documentation.rs
│   ├── handlers/           # Request handlers
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   ├── projects.rs
│   │   ├── build.rs
│   │   ├── testing.rs
│   │   └── documentation.rs
│   └── utils/              # Utility functions
│       ├── mod.rs
│       ├── file_utils.rs
│       └── error.rs
├── examples/               # Example integrations
│   ├── simple_agent.rs
│   └── vscode_extension.rs
└── tests/                 # Integration tests
    └── integration_tests.rs
```

## Getting Started

### Running the MCP Server

```bash
# From the SDK workspace
cargo run -p orbit-mcp

# With custom port and project directory
cargo run -p orbit-mcp -- --port 3000 --project /path/to/project
```

### Running Both Development and MCP Servers

For convenience, a script is provided to run both servers:

```bash
# From the SDK workspace
./scripts/orbit-dev-mcp.sh

# With custom ports
./scripts/orbit-dev-mcp.sh --dev-port 8000 --mcp-port 3000
```

## API Overview

### HTTP API

- `GET /api/status` - Get server status
- `GET /api/docs` - Get server documentation
- `GET /api/components` - List all components
- `POST /api/components` - Create a new component
- `GET /api/components/:id` - Get details of a specific component
- `PUT /api/components/:id` - Update a component
- `DELETE /api/components/:id` - Delete a component
- `GET /api/project` - Get project information
- `POST /api/project/build` - Build the project
- `POST /api/project/test` - Run tests

### WebSocket API (JSON-RPC)

Connect to `ws://{host}:{port}/api/ws` to access the JSON-RPC API:

- `component.list` - List all components
- `component.create` - Create a new component
- `component.get` - Get component details
- `component.update` - Update a component
- `component.delete` - Delete a component
- `component.analyze` - Analyze a component
- `project.info` - Get project information
- `project.create` - Create a new project
- `project.update` - Update project configuration
- `build.compile` - Compile the project
- `build.errors` - Get compilation errors
- `test.run` - Run tests
- `test.results` - Get test results
- `docs.query` - Query documentation
- `docs.examples` - Get examples

## Examples

### Running the Simple Agent Example

```bash
# Start the MCP server
cargo run -p orbit-mcp

# In a separate terminal, run the example
cargo run -p orbit-mcp --example simple_agent
```

### Running the VS Code Extension Example

```bash
# Start the MCP server
cargo run -p orbit-mcp

# In a separate terminal, run the example
cargo run -p orbit-mcp --example vscode_extension
```

## Documentation

Comprehensive documentation for the MCP server is available in:

- `/docs/tooling/orbit-mcp.md` - General MCP server documentation
- API documentation is available through the `/api/docs` endpoint when the server is running

## Future Work

- Implement the remaining API endpoints with real functionality
- Add authentication and authorization mechanisms
- Add support for more IDE integrations
- Enhance integration with existing Orbit tooling
- Improve documentation and examples
