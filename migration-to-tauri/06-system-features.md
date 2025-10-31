# Step 6: System Features (Tray, Updater, Links)

## Overview
Migrate system-level features like system tray, auto-updater, and external link opening.

## Tasks

### 6.1 System Tray
Migrate `original/src/main/system/tray.ts`:
- Use `tauri-plugin-tray` or implement native tray
- Create system tray with icon
- Implement tray menu items
- Handle tray click events
- Copy tray icon templates to appropriate location

### 6.2 Auto-Updater
Migrate `original/src/main/system/updater.ts`:
- Use `tauri-plugin-updater` for update functionality
- **Configure GitHub releases** (matching original app):
  - Repository: `tweakphp/tweakphp`
  - Provider: GitHub releases
  - Use same update mechanism as original
- Implement update checking on app start
- Handle update available notifications
- Implement download and install flow
- Update frontend update store to use Tauri events
- Handle `update.check`, `update.download` commands

### 6.3 External Link Opening
Migrate `original/src/main/system/link.ts`:
- Use `tauri-plugin-opener` (already installed)
- Replace `shell.openExternal()` calls
- Handle URL opening in browser

### 6.4 Window Management
Migrate window-related operations:
- Window size/position saving (already handled in settings)
- Window show/hide from tray
- Window minimize/maximize
- Title bar customization
- Update `tauri.conf.json` window configuration:
  - Min width/height (1100x700)
  - Default size from settings
  - Title bar style
  - Window icon

### 6.5 Notifications (if needed)
- Check if app uses system notifications
- Use `tauri-plugin-notification` if needed
- Replace Electron Notification API

### 6.6 App Lifecycle
- Handle app ready event
- Handle app close events (save state, cleanup)
- Handle window close events
- Implement graceful shutdown

## Deliverables
- ✅ System tray works on all platforms
- ✅ Auto-updater checks and downloads updates
- ✅ External links open in system browser
- ✅ Window size/position persists
- ✅ App lifecycle events handled correctly
- ✅ All system integrations work on macOS, Linux, Windows

## Notes
- Tauri tray plugin may need different configuration per platform
- Auto-updater setup may require signing certificates for distribution
- Window customization in Tauri is different from Electron (check Tauri 2.0 docs)
- Test system tray on all platforms (behavior differs)
- Ensure app doesn't quit when window closes (if tray icon is present)
