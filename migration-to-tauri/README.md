# Migration Plans - Electron to Tauri

## Quick Reference

This directory contains the complete migration plan to convert the Electron.js application to Tauri. The migration is organized into 10 sequential steps.

## Key Decisions Made

Based on your requirements, the following decisions have been incorporated into the plans:

1. **Database**: Using Tauri SQL Plugin with SQLite backend
2. **Client Migration**: Full migration of all client types (Local, SSH, Docker, Kubernetes, Vapor)
3. **LSP Integration**: Intelephense runs as Node.js process via WebSocket bridge
4. **Auto-Updates**: GitHub releases (tweakphp/tweakphp repository)
5. **Primary Platform**: macOS (Intel and Apple Silicon)

## Migration Steps

1. **[01-project-setup-and-dependencies.md](./01-project-setup-and-dependencies.md)** - Install dependencies and configure build
2. **[02-frontend-code-migration.md](./02-frontend-code-migration.md)** - Copy and adapt frontend code
3. **[03-ipc-communication-layer.md](./03-ipc-communication-layer.md)** - Replace Electron IPC with Tauri commands
4. **[04-file-system-operations.md](./04-file-system-operations.md)** - Migrate file operations
5. **[05-database-and-settings.md](./05-database-and-settings.md)** - Implement database and settings in Rust
6. **[06-system-features.md](./06-system-features.md)** - Tray, updater, and system integrations
7. **[07-client-connections.md](./07-client-connections.md)** - All client types (SSH, Docker, etc.)
8. **[08-lsp-integration.md](./08-lsp-integration.md)** - Intelephense LSP server
9. **[09-build-configuration.md](./09-build-configuration.md)** - Build and distribution setup
10. **[10-testing-and-validation.md](./10-testing-and-validation.md)** - Comprehensive testing

## Important Notes

### Intelephense LSP Server
- Intelephense is an npm package that must be bundled or Node.js must be installed
- Architecture: Express HTTP server (port 54331) → WebSocket upgrade → Bridge to Intelephense stdio
- Monaco Editor connects via WebSocket (no frontend changes needed)
- Requires WebSocket server implementation in Rust (tokio-tungstenite or similar)

### Database
- Using Tauri SQL Plugin with SQLite
- Database location: `~/.tweakphp/tweakphp.db`
- Migrations from `original/migrations/` need to be run

### Settings
- Using Tauri Store plugin or JSON file
- Location: `~/.tweakphp/settings.json`
- Includes `intelephenseLicenseKey` field

### Resources to Bundle
- Intelephense npm package
- PHP client PHAR files (client-7.4.phar through client-8.4.phar)
- Laravel ZIP file
- Migration SQL files
- Icons and app resources

## Getting Started

1. Start with Step 1 and work through sequentially
2. Test incrementally after each step
3. Keep the original Electron app running for comparison
4. Refer to each step's deliverables checklist

## Questions or Issues?

If you encounter issues during migration or need clarification on any step, refer back to the specific step's detailed notes section.
