# Step 3: IPC Communication Layer

## Overview
Replace Electron IPC (`ipcRenderer`/`ipcMain`) with Tauri commands and events. Create the communication bridge between frontend and backend.

## Tasks

### 3.1 Create Tauri Command Handlers (Rust)
Create Rust commands in `src-tauri/src/lib.rs` for each IPC channel:

**Init & Settings:**
- `init()` -> returns initial settings
- `store_settings(settings: Settings)` -> stores settings
- `get_platform()` -> returns platform info

**Code History:**
- `code_add(tab_id: u32, code: String, cursor: CursorPosition)` -> adds code history
- `code_undo(tab_id: u32)` -> performs undo
- `code_redo(tab_id: u32)` -> performs redo

**Source/Files:**
- `source_open()` -> opens directory dialog
- `source_open_path(path: String)` -> opens path in system

**Client Operations:**
- `client_connect(connection: ConnectionConfig, data: ConnectionData)` -> connects client
- `client_execute(connection: ConnectionConfig, code: String, loader: String)` -> executes code
- `client_action(connection: ConnectionConfig, action: String)` -> performs client action
- `client_info(connection: ConnectionConfig, loader: String)` -> gets info

**LSP:**
- `lsp_restart()` -> restarts LSP server

**System:**
- `link_open(url: String)` -> opens URL in browser
- `update_check()` -> checks for updates
- `update_download()` -> downloads update

### 3.2 Create TypeScript API Wrapper
Create `src/lib/tauri-api.ts` that wraps Tauri commands:
- Export typed functions matching the original `ipcRenderer.send()` calls
- Export typed event listeners matching the original `ipcRenderer.on()` calls
- Use `invoke()` for commands and `listen()` for events
- Maintain the same API surface as the original Electron IPC handler

### 3.3 Replace IPC Calls in Frontend
Update all components to use new Tauri API:
- Replace `window.ipcRenderer.send()` with Tauri `invoke()` calls
- Replace `window.ipcRenderer.on()` with Tauri `listen()` calls
- Update all listeners in `App.vue` and other components
- Update stores that use IPC (settings, tabs, etc.)

### 3.4 Handle Events and Replies
- Convert one-way IPC messages to Tauri commands with return values
- Convert IPC reply patterns to Tauri command results
- Use Tauri events for push notifications (updates, LSP status, etc.)
- Update event listeners in components

### 3.5 Update Platform Info
- Replace `window.platformInfo.getPlatform()` with Tauri OS plugin
- Use `@tauri-apps/plugin-os` for platform detection

## Deliverables
- ✅ All Rust command handlers created
- ✅ TypeScript API wrapper implemented
- ✅ All frontend IPC calls replaced with Tauri equivalents
- ✅ Event system working (commands can send events back to frontend)
- ✅ Type safety maintained throughout

## Notes
- Start with simple commands first (settings, platform info)
- Commands that require file system, database, or process management will be fully implemented in later steps
- Use serde for JSON serialization between Rust and TypeScript
- Ensure error handling matches original behavior
