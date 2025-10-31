# Step 8: LSP (Language Server Protocol) Integration

## Overview
Migrate the Intelephense LSP server integration from Node.js to Rust.

## Tasks

### 8.1 LSP Server Process Management
Migrate `original/src/main/lsp/index.ts`:
- Use `tauri-plugin-process` or `std::process` to spawn LSP server
- Handle server lifecycle (start, stop, restart)
- Implement process monitoring and error recovery
- Create Rust module `src-tauri/src/lsp/` for LSP management

### 8.2 LSP Server Communication
Migrate `original/src/main/lsp/server.ts` and `server-commons.ts`:
- **Intelephense Setup**: Intelephense is an npm package that runs as a Node.js process
  - Bundle Intelephense with the app or require Node.js installation
  - Resolve Intelephense entry point (similar to `require.resolve('intelephense')`)
  - Spawn Intelephense process with `--stdio` flag via Node.js executable
- **WebSocket Server**: Monaco Editor connects via WebSocket to `ws://127.0.0.1:54331`
  - Use `tokio-tungstenite` or similar for WebSocket server in Rust
  - Use `tokio` HTTP server or `hyper` for HTTP upgrade handling
  - Create Express-like server that handles WebSocket upgrades on port 54331
- **Bridge Pattern**: 
  - Bridge WebSocket messages (from Monaco) to stdio (to Intelephense)
  - Bridge stdio messages (from Intelephense) to WebSocket (to Monaco)
  - Use `vscode-ws-jsonrpc` logic patterns in Rust (may need Rust JSON-RPC crate)
- Handle LSP protocol messages (initialize, textDocument, etc.)
- Set environment variables:
  - `ELECTRON_RUN_AS_NODE: '1'` (or equivalent for Node.js process)
  - `INTELEPHENSE_TELEMETRY_ENABLED: '0'`
  - `INTELEPHENSE_LICENSE_KEY` (from settings)

### 8.3 Monaco Editor Integration
- **No Changes Needed**: Monaco Editor connects via WebSocket to `ws://127.0.0.1:54331`
- The frontend code in `Editor.vue` already handles WebSocket connection
- Ensure Web Worker setup for Monaco works in Tauri (should work identically)
- Monaco uses `MonacoLanguageClient` with WebSocket transport (already configured)
- Test code completion, diagnostics, hover, etc.

### 8.4 Intelephense Integration
- **Intelephense as npm package**: 
  - Intelephense is installed as npm dependency (`intelephense` package)
  - In Tauri, we need to either:
    - Bundle Intelephense with the app (copy from `node_modules/intelephense`)
    - Or require Node.js to be installed and resolve Intelephense at runtime
  - Spawn process: `node <intelephense-entry-point> --stdio`
- Handle Intelephense license key from settings (passed via env var)
- Pass initialization options from Monaco (storage path, license key, file excludes, etc.)
- Handle Intelephense-specific configuration
- Test PHP language features (autocomplete, hover, diagnostics, etc.)

### 8.5 LSP Restart Functionality
- Implement `lsp_restart()` command
- Gracefully shutdown current server
- Start new server instance
- Reconnect Monaco Editor
- Notify frontend of restart status

### 8.6 LSP Configuration
- Handle LSP server configuration
- Pass settings from app settings to LSP server
- Handle workspace/project root configuration
- Manage LSP server options

### 8.7 Error Handling
- Handle LSP server crashes
- Restart server automatically on failure
- Notify user of LSP errors
- Fallback behavior when LSP unavailable

## Deliverables
- ✅ LSP server starts and runs correctly
- ✅ Monaco Editor connects to LSP server
- ✅ Code completion works
- ✅ Diagnostics (errors/warnings) show
- ✅ Hover information works
- ✅ LSP restart works
- ✅ Error recovery works

## Notes
- **Architecture**: Express HTTP server → WebSocket upgrade → Bridge to Intelephense stdio process
- Monaco Editor connects via WebSocket (no changes needed in frontend)
- Need to spawn Node.js process running Intelephense
- Bundle Intelephense from npm package or require Node.js installation
- Use async Rust (tokio) for HTTP/WebSocket server
- Bridge WebSocket messages ↔ stdio streams (bidirectional)
- Test on macOS first (your primary platform), then Linux/Windows
- May need to handle process lifecycle (restart, cleanup on shutdown)
