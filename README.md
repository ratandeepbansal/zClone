# zClone

A fast, smooth ChatGPT clone built with GPUI for macOS.

## Project Status

**Phase 1: Discovery & Foundations** ✅ **COMPLETE**

## Architecture

zClone is organized as a Rust workspace with the following crates:

- **zclone** - Main application entry point
- **zclone-core** - Core data models, session management, and business logic
- **zclone-ui** - UI components and theming (GPUI-based)
- **zclone-api** - OpenAI API integration and streaming

## Requirements

- Rust 1.70+ (2021 edition)
- macOS (primary target platform)
- OpenAI API key (to be configured in settings)

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Development

### Project Structure

```
zclone/
├── crates/
│   ├── zclone-core/    # Data models & session management
│   ├── zclone-ui/      # GPUI components & theming
│   └── zclone-api/     # OpenAI integration
├── src/                # Main application
└── .github/            # CI workflows
```

### Success Metrics

- **Performance**: <16ms render latency (60 FPS)
- **Theme toggle**: <100ms smooth transition
- **Sidebar animation**: <200ms smooth collapse/expand
- **Accessibility**: WCAG 2.1 AA compliant

## Features (Planned)

- ✅ Phase 1: Project scaffolding and architecture
- ⏳ Phase 2: Core architecture & data model
- ⏳ Phase 3: UI/UX system design
- ⏳ Phase 4: Sidebar & session management
- ⏳ Phase 5: Chat surface & composer
- ⏳ Phase 6: OpenAI integration & settings
- ⏳ Phase 7: Theming & visual polish
- ⏳ Phase 8: Performance, QA, and packaging

## License

MIT
