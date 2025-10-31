# Step 7: Client Connections (SSH, Docker, Kubernetes, Vapor, Local)

## Overview
Migrate all client connection logic from Node.js to Rust, including SSH, Docker, Kubernetes, Vapor, and local execution.

## Tasks

### 7.1 Client Interface Design
- Review `original/src/main/client/client.interface.ts`
- Design Rust trait for clients
- Create shared types for connection configs
- Map TypeScript types to Rust structs with serde

### 7.2 Local Client
Migrate `original/src/main/client/local.ts`:
- Execute PHP locally using `tauri-plugin-process` or `std::process::Command`
- Handle PHP execution with Laravel client
- Stream output back to frontend
- Handle errors and timeouts

### 7.3 SSH Client
Migrate `original/src/main/client/ssh.ts`:
- Use Rust SSH library (`thrussh` or `ssh2-rs`)
- Implement SSH connection management
- Handle authentication (password, key-based)
- Execute commands over SSH
- Stream output/errors
- Handle reconnection logic

### 7.4 Docker Client
Migrate `original/src/main/client/docker.ts`:
- Use Docker API via HTTP (rust Docker SDK like `bollard`)
- Or execute `docker` CLI commands
- Handle container operations
- Execute PHP in containers
- Stream output

### 7.5 Kubernetes Client
Migrate `original/src/main/client/kubectl.ts`:
- Execute `kubectl` commands via `std::process::Command`
- Parse kubectl output
- Handle pod/namespace operations
- Stream execution output

### 7.6 Vapor Client
Migrate `original/src/main/client/vapor.ts`:
- Understand Vapor integration (likely HTTP API)
- Use `reqwest` or similar HTTP client in Rust
- Handle authentication
- Execute Vapor-specific operations

### 7.7 Client Base Implementation
Migrate `original/src/main/client/client.base.ts`:
- Shared client functionality
- Connection pooling/management
- Error handling patterns
- Output formatting

### 7.8 Frontend Integration
- Update client commands in Rust:
  - `client_connect()` -> establishes connection
  - `client_execute()` -> executes code
  - `client_action()` -> performs actions
  - `client_info()` -> gets PHP info
- Use Tauri events to stream responses back to frontend
- Handle connection state changes
- Update frontend stores to use new events

### 7.9 Laravel Client Integration
- Handle Laravel client PHAR files
- Ensure client files are accessible
- Execute with appropriate PHP version
- Handle client updates

## Deliverables
- ✅ All client types can connect
- ✅ Code execution works for all connection types
- ✅ Output streams correctly to frontend
- ✅ Error handling works
- ✅ Connection state management works
- ✅ Reconnection logic works

## Notes
- This is the most complex step - client connections involve network operations, process execution, and streaming
- **Migration Order**: Local → SSH → Docker → Kubernetes → Vapor (start simple, build complexity)
- Use async Rust (tokio) for network operations
- Use channels for streaming output from Rust to frontend (via Tauri events)
- Test each client type thoroughly
- May need to bundle PHP executables or kubectl/docker CLI tools
- All client types must be migrated for full feature parity
