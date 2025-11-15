/// Component specifications for zClone GPUI layout hierarchy
///
/// This module defines the core UI components that will be implemented in Phase 4+

use gpui::*;
use zclone_core::*;

/// Root application window component
/// Manages the overall layout and coordinates between sidebar and chat surface
pub struct AppWindow {
    /// Reference to the sidebar component
    sidebar: View<Sidebar>,
    /// Reference to the active chat surface
    chat_surface: View<ChatSurface>,
    /// Reference to settings panel (when visible)
    settings_panel: Option<View<SettingsPanel>>,
    /// Application state
    app_state: Model<AppState>,
}

impl AppWindow {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let app_state = cx.new_model(|_| AppState::default());
            let sidebar = Sidebar::new(app_state.clone(), cx);
            let chat_surface = ChatSurface::new(app_state.clone(), cx);

            Self {
                sidebar,
                chat_surface,
                settings_panel: None,
                app_state,
            }
        })
    }

    pub fn toggle_sidebar(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 4
    }

    pub fn show_settings(&mut self, cx: &mut ViewContext<Self>) {
        if self.settings_panel.is_none() {
            self.settings_panel = Some(SettingsPanel::new(self.app_state.clone(), cx));
        }
    }

    pub fn hide_settings(&mut self, _cx: &mut ViewContext<Self>) {
        self.settings_panel = None;
    }
}

impl Render for AppWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .child(self.sidebar.clone())
            .child(self.chat_surface.clone())
            .when_some(self.settings_panel.clone(), |el, panel| {
                el.child(panel)
            })
    }
}

/// Sidebar component with session list and controls
pub struct Sidebar {
    app_state: Model<AppState>,
    /// Whether the sidebar is collapsed
    is_collapsed: bool,
    /// Search filter text
    search_filter: String,
    /// Scroll offset for virtualization
    scroll_offset: f32,
}

impl Sidebar {
    pub fn new(app_state: Model<AppState>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self {
            app_state,
            is_collapsed: false,
            search_filter: String::new(),
            scroll_offset: 0.0,
        })
    }

    pub fn toggle_collapse(&mut self, _cx: &mut ViewContext<Self>) {
        self.is_collapsed = !self.is_collapsed;
    }

    pub fn create_new_session(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 4
    }

    pub fn filter_sessions(&mut self, filter: String, _cx: &mut ViewContext<Self>) {
        self.search_filter = filter;
    }
}

impl Render for Sidebar {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(280.0))
            .h_full()
            .bg(rgb(0x1e1e1e))
            .when(self.is_collapsed, |el| el.w(px(0.0)))
            .child(
                // Toolbar with new chat button
                div()
                    .h(px(48.0))
                    .child("New Chat Button")
            )
            .child(
                // Search/filter input
                div()
                    .h(px(40.0))
                    .child("Search input")
            )
            .child(
                // Session list (virtualized)
                div()
                    .flex_1()
                    .child("Session list items")
            )
    }
}

/// Session list item component
pub struct SessionListItem {
    session: ChatSession,
    is_active: bool,
}

impl SessionListItem {
    pub fn new(session: ChatSession, is_active: bool) -> Self {
        Self { session, is_active }
    }
}

impl Render for SessionListItem {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(12.0))
            .rounded(px(8.0))
            .when(self.is_active, |el| el.bg(rgb(0x2a2a2a)))
            .child(
                // Title
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.session.title.clone())
            )
            .child(
                // Preview
                div()
                    .text_xs()
                    .text_color(rgb(0x888888))
                    .child(self.session.preview.clone().unwrap_or_default())
            )
    }
}

/// Main chat surface component with message list and composer
pub struct ChatSurface {
    app_state: Model<AppState>,
    /// Scroll position in message list
    scroll_position: f32,
    /// Current composer text
    composer_text: String,
}

impl ChatSurface {
    pub fn new(app_state: Model<AppState>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self {
            app_state,
            scroll_position: 0.0,
            composer_text: String::new(),
        })
    }

    pub fn send_message(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 5
    }

    pub fn scroll_to_bottom(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 5
    }
}

impl Render for ChatSurface {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(rgb(0x1a1a1a))
            .child(
                // Message list (virtualized)
                div()
                    .flex_1()
                    .child("Message list")
            )
            .child(
                // Composer
                div()
                    .h(px(120.0))
                    .child(Composer::new())
            )
    }
}

/// Message bubble component
pub struct MessageBubble {
    message: Message,
}

impl MessageBubble {
    pub fn new(message: Message) -> Self {
        Self { message }
    }
}

impl Render for MessageBubble {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let bg_color = match self.message.role {
            Role::User => rgb(0x2a2a2a),
            Role::Assistant => rgb(0x1e1e1e),
            Role::System => rgb(0x3a3a3a),
        };

        div()
            .flex()
            .flex_col()
            .p(px(16.0))
            .bg(bg_color)
            .rounded(px(8.0))
            .child(
                // Role indicator
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(format!("{:?}", self.message.role))
            )
            .child(
                // Content (will support markdown in Phase 5)
                div()
                    .text_sm()
                    .child(self.message.content.clone())
            )
            .when(self.message.is_streaming, |el| {
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x888888))
                        .child("Streaming...")
                )
            })
    }
}

/// Composer component for input
pub struct Composer {
    text: String,
    is_focused: bool,
}

impl Composer {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            is_focused: false,
        }
    }

    pub fn send(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 5
    }
}

impl Default for Composer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Composer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .p(px(16.0))
            .gap(px(12.0))
            .bg(rgb(0x2a2a2a))
            .rounded(px(8.0))
            .child(
                // Multiline text input
                div()
                    .flex_1()
                    .child("Text input area")
            )
            .child(
                // Send button
                div()
                    .w(px(48.0))
                    .h(px(48.0))
                    .child("Send")
            )
    }
}

/// Settings panel component
pub struct SettingsPanel {
    app_state: Model<AppState>,
    /// API key input (masked)
    api_key_input: String,
    /// Whether to show the API key
    show_api_key: bool,
}

impl SettingsPanel {
    pub fn new(app_state: Model<AppState>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self {
            app_state,
            api_key_input: String::new(),
            show_api_key: false,
        })
    }

    pub fn save_settings(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 6
    }

    pub fn test_connection(&mut self, _cx: &mut ViewContext<Self>) {
        // Implementation in Phase 6
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .absolute()
            .top(px(0.0))
            .right(px(0.0))
            .w(px(400.0))
            .h_full()
            .bg(rgb(0x252525))
            .p(px(24.0))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .child("Settings")
            )
            .child(
                // API Key section
                div()
                    .mt(px(24.0))
                    .child("API Key Input")
            )
            .child(
                // Model selection
                div()
                    .mt(px(16.0))
                    .child("Model selector")
            )
            .child(
                // Theme toggle
                div()
                    .mt(px(16.0))
                    .child("Theme toggle")
            )
    }
}
