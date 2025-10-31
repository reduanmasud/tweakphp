# Step 2: Frontend Code Migration

## Overview
Copy and adapt all frontend/renderer code from the Electron app, removing Electron-specific code and preparing for Tauri integration.

## Tasks

### 2.1 Copy Renderer Source Files
- Copy `original/src/renderer/` to `src/` (adapting structure)
- Copy all Vue components from `original/src/renderer/components/`
- Copy all views from `original/src/renderer/views/`
- Copy all stores from `original/src/renderer/stores/`
- Copy router configuration
- Copy utility files and type definitions

### 2.2 Remove Electron-Specific Code
- Replace `window.ipcRenderer` calls with placeholder comments (we'll implement Tauri commands in Step 3)
- Remove any direct Electron imports
- Update type definitions:
  - Remove `Window.ipcRenderer` interface
  - Add Tauri API types
  - Update platform info access

### 2.3 Update Import Paths
- Adjust import paths to match new project structure
- Ensure `@` alias works correctly
- Fix any relative path imports

### 2.4 Update Entry Point
- Adapt `src/main.ts` to match original `original/src/renderer/main.ts`
- Ensure Monaco Editor worker setup works with Tauri
- Verify Pinia, Vue Router, and other plugins initialize correctly

### 2.5 Copy Assets and Styles
- Copy CSS files (`main.css`, `sf-dump.css`)
- Copy Monaco editor themes
- Copy JavaScript files (`sf-dump.js`)
- Ensure Tailwind CSS configuration is migrated

### 2.6 Update Type Definitions
- Create Tauri-compatible type definitions
- Replace `vite.d.ts` with Tauri-compatible declarations
- Update component prop types if needed

## Deliverables
- ✅ All frontend code copied and adapted
- ✅ No Electron references remain in frontend code
- ✅ All components render without errors
- ✅ TypeScript compiles successfully
- ✅ Application UI matches original (before backend connection)

## Notes
- Keep UI code 100% identical except for IPC communication layer
- Don't implement Tauri commands yet - just prepare the code structure
- Use placeholder functions or TODO comments where IPC calls were made
- Test that the app loads and displays correctly (even if backend features don't work yet)
