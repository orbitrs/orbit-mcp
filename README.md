# Orbit MCP Server

A Model Context Protocol (MCP) server for the Orbit Framework that exposes development APIs to AI agents.

## Overview

The Orbit MCP Server allows AI agents to interact with the Orbit Framework for various tasks, including:

- Creating and modifying components
- Managing projects
- Building and compiling
- Running tests
- Accessing documentation
- Generating code

## Installation

```bash
cargo install --path .
```

## Usage

### Starting the MCP Server

```bash
orbit-mcp --port 3000
```

### Command-line options

- `--port <PORT>`: Set the port for the MCP server (default: 3000)
- `--host <HOST>`: Set the host address (default: 127.0.0.1)
- `--project <PATH>`: Set the path to the Orbit project (default: current directory)

## API Documentation

The MCP server exposes the following API endpoints:

### HTTP API (REST)

- `GET /api/status`: Get server status
- `GET /api/docs`: Get server documentation

### WebSocket API (JSON-RPC)

Connect to `ws://{host}:{port}/api/ws` to access the WebSocket API.

#### Component Management

- `component.create`: Create a new component
- `component.update`: Update an existing component
- `component.delete`: Delete a component
- `component.list`: List available components
- `component.analyze`: Analyze a component

#### Project Management

- `project.create`: Create a new project
- `project.update`: Update project configuration
- `project.info`: Get project information

#### Build and Compilation

- `build.compile`: Compile the project
- `build.errors`: Get compilation errors

#### Testing

- `test.run`: Run tests
- `test.results`: Get test results

#### Documentation

- `docs.query`: Query documentation
- `docs.examples`: Get examples

#### Code Generation

- `code.generate`: Generate code based on specifications
- `code.suggest`: Get code suggestions

## Integration with AI Agents

This MCP server is designed to be easily integrated with AI agents. See the examples directory for sample integrations.

## License

MIT
