# Testing Phase 1 Locally

## Quick Verification Steps

### 1. Check Project Structure
```bash
# Verify all crates exist
ls -la crates/

# Should show:
# - zclone-core/
# - zclone-ui/
# - zclone-api/
```

### 2. Verify Compilation
```bash
# Check that the project compiles
cargo check

# Expected output: "Finished ..." with no errors
```

### 3. Run Unit Tests (when added in Phase 2+)
```bash
cargo test
```

### 4. Check Formatting
```bash
# Verify code is properly formatted
cargo fmt --check
```

### 5. Run Linter
```bash
# Check for common issues
cargo clippy
```

### 6. Build the Application
```bash
# Build in debug mode (faster)
cargo build

# Or build optimized release
cargo build --release
```

### 7. Run the Application
```bash
# Run the app (will show Phase 1 placeholder window)
cargo run

# You should see a GPUI window with:
# - Dark background
# - Text saying "zClone - Phase 1 Complete"
```

### 8. Verify Dependencies
```bash
# Check all dependencies resolve
cargo tree | grep gpui

# Should show GPUI and its dependencies
```

### 9. Check Workspace Structure
```bash
# Verify workspace members
cargo metadata --format-version 1 | jq '.workspace_members'

# Should show all 3 crates
```

## Expected Results

After running `cargo check`, you should see:
```
    Checking zclone-core v0.1.0
    Checking zclone-api v0.1.0
    Checking zclone-ui v0.1.0
    Checking zclone v0.1.0
    Finished ...
```

After running `cargo run`, you should see:
- A native macOS window opens
- Dark background (#1e1e1e)
- White text in the center: "zClone - Phase 1 Complete"

## Troubleshooting

### If cargo check fails:
1. Ensure Rust is up to date: `rustup update`
2. Check GPUI can be cloned: Network access to GitHub
3. Review error messages for missing dependencies

### If cargo run doesn't show a window:
1. Check you're on macOS (GPUI is macOS-first)
2. Verify display permissions if prompted
3. Check console output for errors

### If builds are slow:
- GPUI is large (~500+ dependencies)
- First build can take 5-10 minutes
- Subsequent builds are much faster (cached)
- Use `cargo check` for faster iteration

## Phase 1 Deliverables Checklist

- [ ] Project compiles without errors (`cargo check`)
- [ ] All workspace crates are present (core, ui, api)
- [ ] CI configuration exists (`.github/workflows/ci.yml`)
- [ ] Core data models compile (Message, ChatSession, etc.)
- [ ] Theme system exists (dark/light themes)
- [ ] Application runs and shows window
- [ ] README.md documents the project
- [ ] PHASE1_SUMMARY.md exists with full details

## Quick Start Command

```bash
# One command to verify everything
cargo check && echo "✅ Phase 1 verification complete!"
```
