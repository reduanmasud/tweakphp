# Instructions for Continuing the Migration

## Quick Start for New Agent

You are taking over a migration project from Electron.js to Tauri. The current state is documented in `MIGRATION_STATUS.md`.

## What to Read First

1. **`MIGRATION_STATUS.md`** - Current state and what's been completed
2. **`migration-plans/00-overview.md`** - Overall migration strategy
3. **`migration-plans/02-frontend-code-migration.md`** - Next step details

## Current Status

- ✅ **Step 1 Complete**: Dependencies installed, configuration done, assets copied
- 🔄 **Next Step**: Step 2 - Frontend Code Migration

## How to Continue

### Step 1: Understand the Structure

```bash
# Original Electron app location
original/src/renderer/     # Source of truth for frontend code

# Current Tauri app location
src/                       # Where you'll migrate to

# Migration plans
migration-plans/           # Detailed step-by-step plans
```

### Step 2: Review the Original Code

Before migrating, familiarize yourself with:

- `original/src/renderer/App.vue` - Main app component
- `original/src/renderer/components/` - Vue components
- `original/src/renderer/views/` - Page views
- `original/src/renderer/stores/` - Pinia stores
- `original/src/renderer/router/` - Vue Router config

### Step 3: Follow Step 2 Plan

Open `migration-plans/02-frontend-code-migration.md` and follow it step-by-step:

1. Copy renderer source files from `original/src/renderer/` to `src/`
2. Remove Electron-specific code (IPC calls)
3. Update import paths
4. Update entry point (`main.ts`)
5. Replace the temporary `App.vue`

### Step 4: Testing

After completing Step 2, verify:

- App loads without errors
- UI components render (even if not functional)
- No TypeScript errors
- Monaco Editor can be imported

```bash
npm run dev  # Test Vite dev server
```

## Important Guidelines

### Do's ✅

- Keep UI code identical to original
- Use placeholder comments for IPC calls (e.g., `// TODO: Replace with Tauri command`)
- Test after major file migrations
- Follow the migration plan order
- Preserve file structure as much as possible

### Don'ts ❌

- Don't implement Tauri commands in Step 2 (that's Step 3)
- Don't remove IPC call locations - just comment them out
- Don't change UI logic or component structure
- Don't skip steps or jump ahead

## Key Files to Be Migrated in Step 2

### High Priority

- `original/src/renderer/App.vue` → `src/App.vue`
- `original/src/renderer/main.ts` → `src/main.ts`
- `original/src/renderer/router/index.ts` → `src/router/index.ts`
- All components in `original/src/renderer/components/`
- All views in `original/src/renderer/views/`
- All stores in `original/src/renderer/stores/`

### Supporting Files

- Type definitions: `original/src/renderer/*.d.ts`
- Utility files: `original/src/renderer/*.ts` (not main.ts)
- Events: `original/src/renderer/events.ts`
- Editor setup: `original/src/renderer/editor.ts`

## Common Issues & Solutions

### Issue: Import path errors after copying

**Solution**: Update imports to use `@` alias or relative paths based on new structure

### Issue: Type errors with `window.ipcRenderer`

**Solution**: Comment out IPC calls and add type guards (will be replaced in Step 3)

### Issue: Monaco Editor not working

**Solution**: Ensure worker setup matches original implementation

## What Success Looks Like

After Step 2, you should have:

- ✅ All frontend files copied and adapted
- ✅ App loads in browser (via `npm run dev`)
- ✅ UI matches original (visually)
- ✅ No compilation errors
- ✅ IPC calls commented/prepared for Step 3

## Next Steps After Step 2

Once Step 2 is complete:

1. Update `MIGRATION_STATUS.md` to mark Step 2 as complete
2. Proceed to Step 3: IPC Communication Layer
3. Implement Tauri commands to replace Electron IPC

## Guidelines to Follow

**IMPORTANT**: Read `MIGRATION_GUIDELINES.md` before starting work!

This document contains:

- Code style and conventions
- Documentation requirements
- Testing practices
- Git/version control practices
- IPC migration patterns
- Progress tracking guidelines

## Getting Help

- **Guidelines**: `MIGRATION_GUIDELINES.md` - Essential reading!
- **Migration Plans**: See `migration-plans/` for detailed guidance
- **Original Code**: Reference `original/` for exact implementation details
- **Tauri Docs**: <https://tauri.app/v2/>

## Project Context

This is a PHP development tool called TweakPHP that:

- Provides a code editor (Monaco Editor)
- Executes PHP code via various clients (Local, SSH, Docker, Kubernetes, Vapor)
- Uses Intelephense LSP for PHP language support
- Manages projects and code history
- Runs on macOS, Linux, and Windows

The migration goal is to replace Electron with Tauri while keeping the UI identical.

---

**Ready to continue? Start with Step 2!**

Read `migration-plans/02-frontend-code-migration.md` for detailed instructions.
