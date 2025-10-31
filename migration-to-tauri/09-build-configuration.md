# Step 9: Build Configuration and Distribution

## Overview
Configure Tauri build system to match the original Electron app's build and distribution setup.

## Tasks

### 9.1 Tauri Configuration
Update `src-tauri/tauri.conf.json`:
- Set app metadata:
  - Name: "TweakPHP"
  - Version: Match original version
  - Identifier: `com.tweakphp.app` (matching original)
- Configure window settings:
  - Title: "TweakPHP"
  - Min dimensions: 1100x700
  - Default size from settings (1100x700)
  - Title bar style (hidden inset for macOS)
  - Icon configuration
- Configure bundle settings:
  - Icons for all platforms
  - Bundle identifier: `com.tweakphp.app`
  - App category: Development
- Configure security/permissions for:
  - File system access
  - Network access (SSH, HTTP)
  - Process execution
  - Shell access

### 9.2 Resource Files
- Copy icons from `original/build/` to `src-tauri/icons/`
- Ensure all required icon formats are present (PNG, ICO, ICNS)
- Copy entitlements file for macOS (`original/build/entitlements.mac.plist`)
- Handle app icons for all platforms
- **Bundle Intelephense**: Include Intelephense npm package in bundle resources
- Bundle PHP client PHAR files (from `original/public/`)
- Bundle Laravel ZIP (from `original/public/laravel.zip`)

### 9.3 Extra Resources
- Configure PHP client PHAR files in bundle (client-7.4.phar through client-8.4.phar)
- Configure Laravel ZIP in bundle
- Set up migrations folder in bundle
- Bundle Intelephense npm package for LSP server
- Use Tauri's resource embedding or `extraResources` for extra files
- Ensure Node.js executable is available (or bundle Node.js runtime)

### 9.4 Build Scripts
Update `package.json` scripts:
- `dev` -> `tauri dev`
- `build` -> `tauri build`
- `build-mac` -> `tauri build --target universal-apple-darwin` (or appropriate)
- `build-linux` -> `tauri build --target x86_64-unknown-linux-gnu` (or appropriate)
- `build-win` -> `tauri build --target x86_64-pc-windows-msvc` (or appropriate)

### 9.5 Platform-Specific Configuration
- **macOS (Primary Platform)**:
  - Hardened runtime enabled
  - Entitlements from `original/build/entitlements.mac.plist`
  - Notarization setup (if distributing)
  - Universal binary support (Intel + Apple Silicon)
  - Dark mode support
  - Gatekeeper configuration
- Linux:
  - Desktop file configuration
  - Package dependencies (zlib, etc.)
  - Icon installation
  - Target formats: AppImage, deb, rpm, snap
- Windows:
  - Icon configuration
  - NSIS installer settings (if using)
  - Target architectures: ia32, x64

### 9.6 Update Scripts
- Migrate version update script if needed
- Update client update script for Tauri
- Handle version bumping

### 9.7 Development vs Production
- Handle development mode detection
- Configure different paths for dev/prod
- Handle Laravel client path in both modes
- Ensure resources are accessible in both modes

### 9.8 Bundle Output
- Configure output directory
- Ensure build artifacts are correct
- Test that all resources are included in bundle

## Deliverables
- ✅ `tauri.conf.json` fully configured
- ✅ All icons and resources included in bundle
- ✅ Build scripts work for all platforms
- ✅ App bundles correctly for macOS, Linux, Windows
- ✅ Development and production builds work
- ✅ Output matches original app structure (where applicable)

## Notes
- Tauri 2.0 build configuration may differ from Tauri 1.x
- Test builds on target platforms
- Ensure file sizes are reasonable (Tauri apps should be smaller than Electron)
- Consider code signing for distribution
- May need to adjust resource paths for different platforms
