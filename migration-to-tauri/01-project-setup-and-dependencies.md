# Step 1: Project Setup and Dependencies

## Overview

Set up the Tauri project structure and migrate all necessary dependencies from the Electron app.

## Tasks

### 1.1 Install Frontend Dependencies

- Copy all frontend dependencies from `original/package.json` to the root `package.json`
- Key dependencies to migrate:
  - Vue 3 and related packages (`vue`, `vue-router`, `pinia`)
  - Monaco Editor and related packages (`monaco-editor`, `monaco-languageclient`, etc.)
  - UI libraries (`@headlessui/vue`, `@heroicons/vue`, `reka-ui`)
  - Utilities (`splitpanes`, `vue-tippy`, `zod`)
  - All other renderer-only dependencies

### 1.2 Install Tauri-Specific Dependencies

- Ensure `@tauri-apps/api` is installed
- Add Tauri plugins as needed:
  - `@tauri-apps/plugin-dialog` (for file dialogs)
  - `@tauri-apps/plugin-fs` (for file operations)
  - `@tauri-apps/plugin-shell` (for opening external paths)
  - `@tauri-apps/plugin-os` (for platform info)
  - `@tauri-apps/plugin-store` (for settings storage)
  - `@tauri-apps/plugin-sql` (for SQLite database)
  - `@tauri-apps/plugin-process` (for process management)
  - `@tauri-apps/plugin-notification` (for notifications)
  - `@tauri-apps/plugin-updater` (for auto-updates)

### 1.3 Update Rust Dependencies (Cargo.toml)

- Add required Rust crates:
  - `tauri-plugin-dialog`
  - `tauri-plugin-fs`
  - `tauri-plugin-shell`
  - `tauri-plugin-os`
  - `tauri-plugin-store`
  - `tauri-plugin-sql` (with SQLite support)
  - `tauri-plugin-process`
  - `tauri-plugin-notification`
  - `tauri-plugin-updater`
  - `rusqlite` (if needed for direct SQLite access)
  - `serde` and `serde_json` (already present)

### 1.4 Configure Vite

- Update `vite.config.ts` to match original structure:
  - Set correct port (4999 or use env variable)
  - Configure path aliases (`@` -> `src/renderer/`)
  - Set proper base path
  - Configure build output directory

### 1.5 Update TypeScript Configuration

- Copy and adapt `tsconfig.json` from original
- Ensure proper path mappings
- Configure module resolution

### 1.6 Copy Asset Files

- Copy all assets from `original/src/renderer/assets/` to appropriate location
- Copy icons and build resources
- Copy public files (PHP clients, Laravel zip)

## Deliverables

- ✅ All frontend dependencies installed
- ✅ All Tauri plugins installed and configured
- ✅ Vite and TypeScript properly configured
- ✅ Assets copied to new project structure
- ✅ Project builds successfully with `npm run tauri dev`

## Notes

- Remove Electron-specific dependencies (`electron`, `electron-builder`, etc.)
- Keep all Vue, Monaco, and UI dependencies identical to original
- Ensure Monaco Editor works with Tauri (may need Web Worker adjustments)
