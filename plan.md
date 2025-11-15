# zClone Delivery Plan

## Phase 1 – Discovery & Foundations
1. Validate requirements from `hello.md`/`full_prompt.md`; confirm macOS-first, GPUI-based scope.
2. Inventory GPUI capabilities (multi-window, theming, sidebar, persistence) and identify gaps needing custom code.
3. Define success metrics (render latency, smooth toggle of dark/light, chat persistence reliability).
4. Set up project scaffolding: repo structure, Rust workspace, CI basics.

## Phase 2 – Core Architecture & Data Model
1. Map application state: sessions, messages, sidebar metadata, OpenAI credentials, theme settings.
2. Design persistence layer (local file/SQLite) for chat history + secure storage for API key.
3. Establish messaging pipeline abstraction (request queue, streaming responses, cancellation hooks).
4. Draft GPUI layout hierarchy (root window, sidebar component, chat surface component, settings sheet).

## Phase 3 – UI/UX System Design
1. Define typography stack (SF Pro Display/Text + SF Mono for code) and scaling rules.
2. Specify color palettes for dark/light modes with target contrast ratios; document surface/elevation tokens.
3. Create component specs: message bubble, composer, toolbar, collapsible sidebar, settings panel, toasts.
4. Outline motion/interaction guidelines (sidebar slide, theme toggle fade, message streaming anims).

## Phase 4 – Sidebar & Session Management
1. Implement GPUI sidebar component with collapse/expand animation and keyboard shortcuts.
2. Build session list item (title, preview, timestamp, active indicator, context menu for rename/delete).
3. Hook sidebar actions to persistence: create, duplicate, archive chats; auto-save ordering.
4. Add search/filter for sessions; ensure lazy loading for large histories.

## Phase 5 – Chat Surface & Composer
1. Implement virtualized message list with role-based styling, markdown/rendering, and code block mono font.
2. Add streaming response renderer with token-by-token updates and inline retry/edit controls.
3. Build composer: multiline input, attachments placeholder, send button with shortcut, token counter.
4. Ensure layout responsiveness for different window sizes and split-view (future multi-chat) scenarios.

## Phase 6 – OpenAI Integration & Settings
1. Create secure settings panel (modal or dedicated view) with masked API key field + validation feedback.
2. Store key encrypted/obfuscated locally; expose key availability state to composer.
3. Wire OpenAI client (configurable model, temperature, system prompt presets) with error handling.
4. Add connection test workflow and surfaced errors to settings + chat UI to guide user actions.

## Phase 7 – Theming & Visual Polish
1. Implement theme manager for dark/light palettes, persisting user choice + auto mode (follow macOS).
2. Apply typography tokens and spacing ramp consistently across components.
3. Add micro-interactions (hover states, button press depth, smooth transitions) optimized for GPUI.
4. Audit accessibility: contrast checks, focus outlines, VoiceOver labels, reduced motion option.

## Phase 8 – Performance, QA, and Packaging
1. Profile render loops and message streaming to keep frame times <16ms; optimize GPUI subscriptions.
2. Add unit/UI tests for state stores, theming toggles, persistence, and OpenAI client fallbacks.
3. Run manual QA scripts covering sidebar, chat flows, settings, theme toggles, offline/error states.
4. Package macOS build (codesign/notarize if needed) and update docs with setup + API key instructions.
