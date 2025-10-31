# Migration Guidelines and Best Practices

## Overview

This document outlines the guidelines, conventions, and practices that should be followed during the migration from Electron to Tauri. Following these guidelines ensures consistency, maintainability, and a smooth migration process.

## General Principles

### 1. Preserve Original Behavior
- **Goal**: Keep UI and functionality identical to the original Electron app
- **Approach**: Migrate code faithfully, only change what's necessary for Tauri integration
- **Test**: Compare side-by-side with original app when possible

### 2. Incremental Migration
- **Work step-by-step**: Complete each step fully before moving to the next
- **Test frequently**: Verify changes work after each major update
- **Don't skip steps**: Steps are designed to build on each other

### 3. Document Everything
- **Document changes**: Note any deviations from the plan
- **Record issues**: Log problems and solutions
- **Update status**: Keep `MIGRATION_STATUS.md` current

## Code Style and Conventions

### TypeScript/JavaScript

1. **Follow Original Style**
   - Match the original code style from `original/` directory
   - Use single quotes (as configured in `.prettierrc`)
   - Follow existing indentation and formatting

2. **Naming Conventions**
   - Keep original variable and function names
   - Use descriptive names that match original code
   - Follow Vue 3 composition API patterns

3. **Comments**
   - Add TODO comments for IPC calls that need Tauri implementation:
     ```typescript
     // TODO(Step 3): Replace with Tauri command
     // window.ipcRenderer.send('client.execute', data)
     ```
   - Document any significant changes from original
   - Explain "why" not "what" (code should be self-explanatory)

4. **Type Safety**
   - Preserve all TypeScript types from original
   - Add type definitions for Tauri APIs where needed
   - Use strict TypeScript settings (already configured)

### Rust Code

1. **Code Style**
   - Follow Rust standard formatting (`cargo fmt`)
   - Use `rust-analyzer` for linting (`cargo clippy`)
   - Follow Tauri 2.0 patterns and conventions

2. **Error Handling**
   - Use `Result<T, E>` for fallible operations
   - Provide meaningful error messages
   - Handle errors gracefully and return appropriate responses

3. **Command Functions**
   - Use `#[tauri::command]` attribute for all commands
   - Return `Result<T>` or plain types (Tauri handles serialization)
   - Use descriptive function names matching IPC channel names:
     ```rust
     #[tauri::command]
     async fn client_connect(connection: ConnectionConfig) -> Result<ConnectReply, String> {
         // Implementation
     }
     ```

### Vue Components

1. **Component Structure**
   - Preserve original component structure exactly
   - Keep template, script, and style organization
   - Maintain component props and emits

2. **State Management**
   - Use Pinia stores as in original
   - Preserve store structure and actions
   - Update only to replace IPC calls with Tauri

3. **Monaco Editor**
   - Keep Monaco Editor setup identical
   - Preserve all editor configuration
   - Ensure Web Workers work correctly in Tauri context

## Documentation Requirements

### Required Documentation

1. **Update MIGRATION_STATUS.md**
   - Mark completed steps
   - Document any issues encountered
   - Note deviations from plan
   - Add new file locations if structure changes

2. **Step Completion Notes**
   - For each completed step, document:
     - What was done
     - Any challenges faced
     - Solutions implemented
     - Files created/modified
     - Testing performed

3. **Code Comments**
   - Document complex logic or non-obvious decisions
   - Explain Tauri-specific implementations
   - Note any differences from Electron approach

4. **Issue Logging**
   - Create `MIGRATION_ISSUES.md` if significant issues arise
   - Document problem, solution, and lessons learned

### Documentation Template

When completing a step, add to `MIGRATION_STATUS.md`:

```markdown
## Step X: [Step Name] - Completed [Date]

### Completed Tasks
- [ ] Task 1
- [ ] Task 2

### Files Created
- `path/to/new/file.ts`

### Files Modified
- `path/to/modified/file.ts`

### Issues Encountered
- **Issue**: Description
  - **Solution**: How it was resolved

### Notes
- Any important notes or deviations from plan
```

## Testing Practices

### Testing Checklist

1. **After Each Step**
   - [ ] Code compiles without errors
   - [ ] TypeScript types are correct
   - [ ] No runtime errors in dev mode
   - [ ] UI renders (if applicable)

2. **After Frontend Migration (Step 2)**
   - [ ] App loads in browser
   - [ ] All components render
   - [ ] No console errors (except expected IPC errors)
   - [ ] Monaco Editor initializes

3. **After IPC Layer (Step 3)**
   - [ ] Commands can be called from frontend
   - [ ] Responses are received correctly
   - [ ] Events work (if applicable)

4. **Integration Testing**
   - [ ] Test each feature as it's implemented
   - [ ] Compare behavior with original app
   - [ ] Test edge cases

### Testing Commands

```bash
# Type checking
npm run build  # Runs vue-tsc

# Dev server
npm run dev

# Tauri dev
npm run tauri:dev

# Rust checking
cd src-tauri && cargo check

# Rust clippy
cd src-tauri && cargo clippy
```

## File Organization

### Directory Structure

Maintain this structure:

```
src/
├── components/      # Vue components
├── views/          # Page views
├── stores/         # Pinia stores
├── router/         # Vue Router
├── assets/         # Static assets (CSS, images, themes)
├── lib/            # Utility functions (if needed)
├── types/          # TypeScript type definitions
├── App.vue         # Main app component
└── main.ts         # Entry point

src-tauri/
├── src/
│   ├── commands/   # Tauri command handlers (Step 3+)
│   ├── db/         # Database code (Step 5)
│   ├── client/     # Client connections (Step 7)
│   ├── lsp/        # LSP integration (Step 8)
│   ├── system/     # System features (Step 6)
│   ├── tools/      # Tools (code history, etc.)
│   └── lib.rs      # Main library file
└── Cargo.toml
```

### File Naming

- **Vue components**: PascalCase (e.g., `Editor.vue`, `TitleBar.vue`)
- **TypeScript files**: camelCase (e.g., `codeHistory.ts`, `settings.ts`)
- **Rust files**: snake_case (e.g., `code_history.rs`, `client_connection.rs`)
- **Match original**: When possible, keep same names as original

## Git/Version Control

### Commit Practices

1. **Commit Frequency**
   - Commit after completing major tasks
   - Commit before moving to next step
   - Don't commit incomplete work (use WIP if needed)

2. **Commit Messages**
   ```
   Step X: Brief description
   
   - Detailed change 1
   - Detailed change 2
   - Fixes issue with...
   ```

   Example:
   ```
   Step 1: Project setup and dependencies
   
   - Install all frontend dependencies
   - Configure Vite and TypeScript
   - Copy assets from original app
   - Set up Tauri configuration
   ```

3. **Branch Strategy** (if applicable)
   - Use feature branches for major steps
   - Keep main/master stable
   - Merge after step completion and testing

### What to Commit

- ✅ Configuration files
- ✅ Source code files
- ✅ Documentation updates
- ❌ `node_modules/` (should be in .gitignore)
- ❌ `src-tauri/target/` (should be in .gitignore)
- ❌ Build artifacts

## IPC Migration Pattern

### Standard Pattern for Replacing IPC Calls

**Original Electron Pattern:**
```typescript
// Frontend
window.ipcRenderer.send('command.name', data)
window.ipcRenderer.on('command.reply', (data) => { ... })
```

**Tauri Pattern (Step 3):**

**Frontend:**
```typescript
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// Command call
const result = await invoke('command_name', { data })

// Event listener
await listen('command-reply', (event) => { ... })
```

**Rust (Backend):**
```rust
#[tauri::command]
async fn command_name(data: DataType) -> Result<ReturnType, String> {
    // Implementation
    Ok(result)
}

// In lib.rs
.invoke_handler(tauri::generate_handler![command_name])
```

### Step 2 Preparation

In Step 2, prepare IPC calls like this:

```typescript
// TODO(Step 3): Replace with Tauri command
// const result = await invoke('client_connect', { connection })
window.ipcRenderer.send('client.connect', connection)

// Or create a placeholder function:
async function clientConnect(connection: ConnectionConfig) {
  // TODO(Step 3): Implement with Tauri
  console.warn('IPC call not yet implemented')
  // window.ipcRenderer.send('client.connect', connection)
}
```

## Error Handling

### Frontend Error Handling

1. **IPC Call Errors**
   - Provide user-friendly error messages
   - Log errors for debugging
   - Handle async errors with try/catch

2. **Graceful Degradation**
   - If feature isn't ready, show appropriate message
   - Don't break entire app for one missing feature
   - Use feature flags if needed

### Backend Error Handling

1. **Command Errors**
   ```rust
   #[tauri::command]
   async fn my_command() -> Result<SuccessType, String> {
       match operation() {
           Ok(result) => Ok(result),
           Err(e) => Err(format!("Operation failed: {}", e)),
       }
   }
   ```

2. **Validation**
   - Validate inputs in Rust commands
   - Return clear error messages
   - Use serde for automatic validation where possible

## Progress Tracking

### Track Progress In

1. **MIGRATION_STATUS.md**
   - Update after each step
   - Mark tasks as complete
   - Document blockers

2. **TODO Comments in Code**
   - Use TODO comments for future work
   - Reference step number: `// TODO(Step 3): ...`
   - Remove TODOs when completed

3. **Git Commits**
   - Clear commit messages show progress
   - Tags can mark major milestones

## Communication

### When to Document Issues

Document if:
- Step takes significantly longer than expected
- Encounter blocking issues
- Need to deviate from plan
- Discover missing requirements

### Issue Documentation Format

```markdown
## Issue: [Brief Description]

**Step**: Step X
**Date**: YYYY-MM-DD
**Status**: Open | In Progress | Resolved

**Description**:
Detailed description of the issue

**Impact**:
What this affects

**Solution**:
How it was or will be resolved

**Notes**:
Additional context
```

## Code Review Checklist

Before considering a step complete:

- [ ] All tasks from step plan completed
- [ ] Code follows style guidelines
- [ ] No TypeScript/Rust errors
- [ ] Tests pass (if applicable)
- [ ] Documentation updated
- [ ] MIGRATION_STATUS.md updated
- [ ] Code reviewed (self-review minimum)

## Resources

### Documentation
- **Tauri 2.0 Docs**: https://tauri.app/v2/
- **Vue 3 Docs**: https://vuejs.org/
- **Monaco Editor**: https://microsoft.github.io/monaco-editor/
- **Original App**: Reference `original/` directory

### Help and Support
- Check `migration-plans/` for step-specific guidance
- Review `MIGRATION_STATUS.md` for current state
- Reference original code for exact behavior

## Reminders

### Do's ✅
- Follow the step-by-step plan
- Test after major changes
- Document deviations
- Keep UI identical to original
- Use placeholder comments for future work

### Don'ts ❌
- Don't skip steps
- Don't change UI unnecessarily
- Don't implement features out of order
- Don't commit incomplete work without WIP tag
- Don't forget to update documentation

## Issue Logging

When encountering significant issues:

1. **Document in `migration-plans/MIGRATION_ISSUES.md`**
   - Use the template provided
   - Include step, date, status, description, solution, lessons

2. **Update Status**
   - Note the issue in `migration-plans/MIGRATION_STATUS.md`
   - Update status as issue is resolved

3. **Learn and Share**
   - Document solutions for future reference
   - Update guidelines if new patterns emerge

## Questions?

If you encounter something not covered in these guidelines:
1. Check the migration plan for that step
2. Review `migration-plans/MIGRATION_STATUS.md` for context
3. Reference the original code in `original/`
4. Document the question and how you resolved it
5. Update this guide if the answer would help others

---

**Remember**: The goal is a faithful migration that preserves all functionality while leveraging Tauri's benefits. When in doubt, preserve original behavior.

**Documentation is part of the work**: Keep `migration-plans/MIGRATION_STATUS.md` and `migration-plans/MIGRATION_ISSUES.md` updated as you progress.

