# Testing Checklist for TweakPHP Migration

**Date Created**: Current Session  
**Step**: Step 10 - Testing and Validation  
**Status**: In Progress

## Overview

This document provides a comprehensive testing checklist for validating the migrated Tauri application. Each feature should be tested against the original Electron app to ensure feature parity.

## Test Environment Setup

### Prerequisites
- [ ] Node.js installed (for Intelephense LSP)
- [ ] PHP installed locally (for local client testing)
- [ ] SSH access to test server (for SSH client testing)
- [ ] Docker installed (for Docker client testing)
- [ ] kubectl configured (for Kubernetes client testing)
- [ ] Laravel Vapor CLI configured (for Vapor client testing)
- [ ] Original Electron app available for comparison

### Test Data
- [ ] Sample PHP project directory
- [ ] Sample PHP files of various sizes
- [ ] Test SSH credentials
- [ ] Test Docker containers
- [ ] Test Kubernetes cluster/pods

## Feature Testing Checklist

### 1. App Initialization

**Test**: App starts correctly
- [ ] App launches without errors
- [ ] Window appears with correct size (1100x700 minimum)
- [ ] No console errors in dev tools
- [ ] Database initializes correctly (`~/.tweakphp/tweakphp.db`)
- [ ] Settings directory created (`~/.tweakphp/`)
- [ ] LSP server starts automatically (check console for `[LSP] WebSocket server listening`)

**Expected**: App should start smoothly with all systems initialized

---

### 2. Settings Management

**Test**: Settings load and save
- [ ] Settings load correctly on app start (check `init()` command)
- [ ] Settings UI displays current values
- [ ] Changing PHP path saves correctly
- [ ] Changing Laravel path saves correctly
- [ ] Changing theme saves correctly
- [ ] Changing editor font size saves correctly
- [ ] Changing word wrap setting saves correctly
- [ ] Changing Vim mode setting saves correctly
- [ ] All settings persist after app restart
- [ ] Settings file location: `~/.tweakphp/settings.json`

**Test**: PHP Path Validation
- [ ] Setting PHP path to valid executable works
- [ ] Setting PHP path to directory auto-finds `php` executable
- [ ] Setting PHP path to invalid path shows error
- [ ] `settings.php-located` event fires when PHP found in directory
- [ ] PHP path validation works on macOS, Linux, Windows

**Expected**: All settings persist correctly and PHP path validation works as expected

---

### 3. Code Editor (Monaco Editor)

**Test**: Editor Functionality
- [ ] Monaco Editor loads and displays
- [ ] Code can be typed and edited
- [ ] Syntax highlighting works for PHP
- [ ] Editor themes can be changed (all 7 themes available)
- [ ] Font size changes work
- [ ] Word wrap toggle works
- [ ] Line numbers display correctly
- [ ] Editor scrolls correctly
- [ ] Editor works with large files (test with 10k+ lines)

**Test**: Vim Mode
- [ ] Vim mode can be enabled/disabled
- [ ] Vim keybindings work when enabled
- [ ] Vim mode persists across app restarts

**Test**: Editor Themes
- [ ] All editor themes load correctly
- [ ] Theme switching works without errors
- [ ] Theme persists across app restarts

**Expected**: Editor works identically to original Electron app

---

### 4. Code History (Undo/Redo)

**Test**: Basic Undo/Redo
- [ ] Code changes are tracked in history
- [ ] Undo (Cmd/Ctrl+Z) restores previous code state
- [ ] Redo (Cmd/Ctrl+Shift+Z) restores next code state
- [ ] Cursor position is restored after undo/redo
- [ ] Multiple undo/redo operations work correctly

**Test**: History Persistence
- [ ] History persists across tab switches
- [ ] History persists across app restarts
- [ ] History works correctly with multiple tabs
- [ ] History is isolated per tab

**Test**: Edge Cases
- [ ] Undo when no history available shows appropriate message
- [ ] Redo when no future history available shows appropriate message
- [ ] History works with very large code files
- [ ] History branches correctly (undo, make changes, check branching)

**Test**: Database Operations
- [ ] Code history is stored in database correctly
- [ ] Tab states are tracked correctly
- [ ] No database corruption issues
- [ ] Database handles concurrent operations correctly

**Expected**: Undo/redo works identically to original app with proper persistence

---

### 5. Project Management

**Test**: Open Directory
- [ ] Open directory dialog appears
- [ ] Selecting directory loads project correctly
- [ ] Project tiles display correctly
- [ ] Multiple projects can be opened
- [ ] Project context menu works (right-click)
- [ ] "Open in Finder/Explorer" works (tests `source.openPath`)

**Test**: Project Tiles
- [ ] Project tiles display project name
- [ ] Project tiles show correct path
- [ ] Clicking tile opens project
- [ ] Project tiles persist across app restarts

**Expected**: Project management works identically to original app

---

### 6. Client Connections

#### 6.1 Local Client

**Test**: Local Execution
- [ ] Local client connects successfully
- [ ] PHP version is detected correctly
- [ ] Code execution works
- [ ] Output displays correctly
- [ ] Errors display correctly
- [ ] PHP path from settings is used correctly

**Test**: PHAR Client
- [ ] Correct PHAR client is selected based on PHP version
- [ ] PHAR client is found in public directory (dev) or resources (production)
- [ ] PHAR client executes correctly

**Expected**: Local execution works identically to original app

#### 6.2 SSH Client

**Test**: SSH Connection
- [ ] SSH connection with key-based auth works
- [ ] SSH connection with password auth works (if sshpass available)
- [ ] Connection displays connection status
- [ ] PHP version detection on remote server works
- [ ] PHAR client uploads to remote server correctly (`~/.tweakphp/`)
- [ ] Code execution on remote server works
- [ ] Output streams correctly
- [ ] Errors display correctly
- [ ] Disconnection works correctly
- [ ] Reconnection works correctly

**Test**: Error Handling
- [ ] Invalid SSH credentials show error
- [ ] Connection timeout handled gracefully
- [ ] Network errors handled gracefully

**Expected**: SSH client works identically to original app

#### 6.3 Docker Client

**Test**: Docker Connection
- [ ] Docker container connection works
- [ ] PHP version detection in container works
- [ ] PHAR client copied to container correctly
- [ ] Code execution in container works
- [ ] Output displays correctly
- [ ] Container actions work:
  - [ ] `getContainers` action lists containers
  - [ ] `getPHPVersion` action gets PHP version

**Test**: Error Handling
- [ ] Invalid container name shows error
- [ ] Container not running shows error
- [ ] Docker daemon not running shows error

**Expected**: Docker client works identically to original app

#### 6.4 Kubernetes Client

**Test**: Kubernetes Connection
- [ ] Kubernetes pod connection works
- [ ] kubectl context detection works
- [ ] Namespace selection works
- [ ] Pod selection works
- [ ] PHP version detection in pod works
- [ ] PHAR client uploads to pod correctly
- [ ] Code execution in pod works
- [ ] Output displays correctly
- [ ] Kubernetes actions work:
  - [ ] `getContexts` action lists contexts
  - [ ] `getNamespaces` action lists namespaces
  - [ ] `getPods` action lists pods

**Test**: Error Handling
- [ ] Invalid context shows error
- [ ] Pod not found shows error
- [ ] kubectl not configured shows error

**Expected**: Kubernetes client works identically to original app

#### 6.5 Vapor Client

**Test**: Vapor Connection
- [ ] Vapor environment connection works
- [ ] Vapor environments are detected from vapor.yml
- [ ] Code execution via Vapor CLI works
- [ ] Output displays correctly
- [ ] Vapor actions work:
  - [ ] `getEnvironments` action lists environments

**Test**: Error Handling
- [ ] Invalid vapor.yml shows error
- [ ] Vapor CLI not installed shows error
- [ ] Invalid environment shows error

**Expected**: Vapor client works identically to original app

---

### 7. LSP Integration (Intelephense)

**Test**: LSP Server
- [ ] LSP server starts automatically on app launch
- [ ] WebSocket server listens on `ws://127.0.0.1:54331`
- [ ] Monaco Editor connects to LSP server
- [ ] LSP server handles multiple connections correctly

**Test**: Code Completion
- [ ] Code completion appears when typing
- [ ] Code completion shows PHP functions/classes
- [ ] Code completion works with namespaces
- [ ] Code completion works with autoloaded classes

**Test**: Diagnostics
- [ ] PHP syntax errors are highlighted
- [ ] Warnings are displayed
- [ ] Diagnostics update as code changes
- [ ] Error messages are clear and helpful

**Test**: Hover Information
- [ ] Hover shows function/class documentation
- [ ] Hover shows parameter information
- [ ] Hover shows type information

**Test**: LSP Restart
- [ ] LSP restart command works (`lsp.restart`)
- [ ] Editor reconnects after restart
- [ ] LSP functionality works after restart

**Test**: Error Handling
- [ ] Missing Node.js shows helpful error message
- [ ] Missing Intelephense shows helpful error message
- [ ] Connection failures handled gracefully
- [ ] LSP server crashes handled gracefully

**Expected**: LSP integration works identically to original app

---

### 8. System Features

**Test**: Auto-Updater
- [ ] Update check command works (`update.check`)
- [ ] Update download command works (`update.download`)
- [ ] Update events are emitted correctly:
  - [ ] `tauri://update-available` → `update:available`
  - [ ] `tauri://update-not-available` → `update:not-available`
  - [ ] `tauri://update-downloaded` → `update:downloaded`
- [ ] Update UI displays correctly
- [ ] Update installation works

**Test**: Window Management
- [ ] Window show command works
- [ ] Window hide command works
- [ ] Window minimize command works
- [ ] Window maximize command works
- [ ] Window close command works
- [ ] Window size persists across app restarts
- [ ] Window position persists across app restarts

**Test**: External Links
- [ ] External links open in default browser
- [ ] Links open correctly on macOS, Linux, Windows

**Test**: App Lifecycle
- [ ] App closes gracefully
- [ ] Window close event emits `ssh.disconnect` event
- [ ] App saves state before closing
- [ ] App restores state on restart

**Test**: System Tray
- [ ] ⚠️ System tray not yet implemented (Tauri 2.0 tray plugin pending)
- [ ] When implemented: Tray icon displays
- [ ] When implemented: Tray menu works
- [ ] When implemented: Tray actions work

**Expected**: System features work identically to original app (except system tray)

---

### 9. UI/UX Validation

**Test**: Visual Comparison
- [ ] UI matches original Electron app exactly
- [ ] All components render correctly
- [ ] Colors match original app
- [ ] Fonts match original app
- [ ] Spacing matches original app
- [ ] Icons display correctly

**Test**: Themes
- [ ] Light theme works correctly
- [ ] Dark theme works correctly
- [ ] Theme switching works smoothly
- [ ] All UI elements adapt to theme

**Test**: Responsive Behavior
- [ ] Window resizing works correctly
- [ ] Components adapt to window size
- [ ] Minimum window size enforced (1100x700)
- [ ] Editor resizes correctly

**Test**: Accessibility
- [ ] Keyboard shortcuts work correctly
- [ ] Tab navigation works
- [ ] Focus indicators visible

**Expected**: UI/UX matches original app exactly

---

### 10. Edge Cases and Error Handling

**Test**: Invalid Inputs
- [ ] Invalid PHP path handled gracefully
- [ ] Invalid directory path handled gracefully
- [ ] Invalid SSH credentials handled gracefully
- [ ] Invalid Docker container handled gracefully
- [ ] Invalid Kubernetes pod handled gracefully

**Test**: Network Failures
- [ ] SSH connection timeout handled gracefully
- [ ] Network errors show user-friendly messages
- [ ] Connection retry works (if applicable)

**Test**: Database Issues
- [ ] Database corruption handled gracefully
- [ ] Database lock errors handled gracefully
- [ ] Missing database file creates new database

**Test**: Missing Resources
- [ ] Missing PHAR files show helpful error
- [ ] Missing Laravel ZIP shows helpful error
- [ ] Missing Node.js shows helpful error
- [ ] Missing Intelephense shows helpful error

**Test**: Permission Errors
- [ ] File system permission errors handled gracefully
- [ ] Database permission errors handled gracefully
- [ ] User-friendly error messages displayed

**Expected**: All error scenarios handled gracefully with helpful messages

---

### 11. Performance Testing

**Test**: Startup Time
- [ ] App startup time < 3 seconds (cold start)
- [ ] App startup time < 1 second (warm start)
- [ ] Compare with original Electron app (should be faster)

**Test**: Bundle Size
- [ ] macOS app bundle size < 100MB (uncompressed)
- [ ] Compare with original Electron app (should be smaller)
- [ ] DMG size reasonable

**Test**: Large Files
- [ ] Editor works with 10k+ line files
- [ ] Editor works with 100k+ line files
- [ ] History works with large files
- [ ] No memory leaks with large files

**Test**: Memory Usage
- [ ] Memory usage reasonable (< 200MB idle)
- [ ] Memory usage stable over time
- [ ] Compare with original Electron app (should be lower)

**Test**: Many Tabs
- [ ] App handles 10+ open tabs
- [ ] App handles 50+ open tabs
- [ ] Performance remains acceptable

**Expected**: Performance meets or exceeds original Electron app

---

### 12. Cross-Platform Testing

#### macOS (Primary Platform)

**Test**: macOS Intel
- [ ] App runs on Intel Mac
- [ ] All features work correctly
- [ ] Build script works: `npm run build:mac-intel`

**Test**: macOS Apple Silicon
- [ ] App runs on Apple Silicon Mac
- [ ] All features work correctly
- [ ] Build script works: `npm run build:mac-arm`

**Test**: macOS Universal
- [ ] Universal binary works on both architectures
- [ ] Build script works: `npm run build:mac`
- [ ] DMG installs correctly

**Expected**: macOS app works perfectly on both architectures

#### Linux

**Test**: Ubuntu/Debian
- [ ] App runs on Ubuntu/Debian
- [ ] All features work correctly
- [ ] DEB package installs correctly
- [ ] Dependencies resolved correctly (zlib)
- [ ] Build script works: `npm run build:linux`

**Test**: Linux ARM64
- [ ] App runs on ARM64 Linux
- [ ] All features work correctly
- [ ] Build script works: `npm run build:linux-arm`

**Expected**: Linux app works correctly

#### Windows

**Test**: Windows x64
- [ ] App runs on Windows 10/11
- [ ] All features work correctly
- [ ] NSIS installer works correctly
- [ ] App installs to Program Files correctly
- [ ] Build script works: `npm run build:win`

**Expected**: Windows app works correctly

---

### 13. Build Verification

**Test**: Development Build
- [ ] `npm run tauri:dev` works correctly
- [ ] Hot reload works (if applicable)
- [ ] Debug console shows correct output

**Test**: Production Build
- [ ] `npm run tauri:build` works correctly
- [ ] Build completes without errors
- [ ] All resources bundled correctly
- [ ] PHAR files included in bundle
- [ ] Laravel ZIP included in bundle
- [ ] Icons included in bundle

**Test**: Platform-Specific Builds
- [ ] macOS builds work (`build:mac`, `build:mac-intel`, `build:mac-arm`)
- [ ] Linux builds work (`build:linux`, `build:linux-arm`)
- [ ] Windows builds work (`build:win`)

**Test**: Installation
- [ ] macOS DMG installs correctly
- [ ] Linux DEB/RPM installs correctly
- [ ] Windows NSIS installer works correctly
- [ ] App launches after installation
- [ ] All features work after installation

**Expected**: All builds work correctly and install successfully

---

### 14. Migration Testing

**Test**: Settings Migration
- [ ] Settings from Electron app can be imported (if applicable)
- [ ] Settings format is compatible
- [ ] Settings migration path works

**Test**: Database Migration
- [ ] Database migrations run correctly
- [ ] Migration 001 creates tables correctly
- [ ] Migration system handles multiple migrations
- [ ] Database upgrades work correctly

**Expected**: Migration paths work correctly

---

## Test Results Summary

### Passed Tests
- [ ] All critical features tested and passing
- [ ] All edge cases tested and passing
- [ ] Performance meets requirements
- [ ] Cross-platform compatibility verified

### Failed Tests
- [ ] Document any failed tests below
- [ ] Note: Fix bugs before marking step complete

### Known Issues
- [ ] Document any known issues
- [ ] Note: Some issues may be acceptable (e.g., system tray pending)

---

## Testing Notes

### Issues Found
- Document any issues discovered during testing

### Fixes Applied
- Document any fixes applied during testing

### Performance Notes
- Document performance observations
- Compare with original Electron app

### Platform-Specific Notes
- Document any platform-specific findings

---

## Next Steps After Testing

1. Fix any bugs discovered during testing
2. Update documentation with any differences from Electron version
3. Update README with Tauri-specific instructions
4. Prepare for release

---

**Last Updated**: Current Session  
**Status**: In Progress

