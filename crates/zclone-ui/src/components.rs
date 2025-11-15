/// Component implementations for zClone GPUI layout hierarchy
///
/// This module implements the core UI components for Phase 4

use gpui::*;
use gpui::prelude::*;
use zclone_core::*;
use crate::theme::*;

/// Root application window component
pub struct AppWindow {
    app_state: Entity<AppState>,
    sidebar_collapsed: bool,
    theme: ZCloneTheme,
    persistence: Option<SqliteStore>,
    composer_text: String,
}

impl AppWindow {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let app_state = cx.new(|_| AppState::default());

            // Initialize persistence
            let persistence = SqliteStore::default_db_path()
                .ok()
                .and_then(|path| SqliteStore::new(path).ok());

            // Load initial state from persistence
            if let Some(store) = &persistence {
                if let Ok(sessions) = store.load_all_sessions() {
                    app_state.update(cx, |state, _| {
                        state.sessions = sessions;
                        if let Some(first_session) = state.sessions.first() {
                            state.active_session_id = Some(first_session.id.clone());
                        }
                    });
                }

                if let Ok(Some(settings)) = store.load_settings() {
                    app_state.update(cx, |state, _| {
                        state.settings = settings;
                    });
                }
            }

            Self {
                app_state,
                sidebar_collapsed: false,
                theme: ZCloneTheme::dark(),
                persistence,
                composer_text: String::new(),
            }
        })
    }

    pub fn toggle_sidebar(&mut self, _cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
    }

    pub fn create_new_session(&mut self, cx: &mut Context<Self>) {
        self.app_state.update(cx, |state, _| {
            let mut session = ChatSession::new("New Chat");

            // Add welcome messages for demonstration
            session.messages.push(Message::new(
                Role::System,
                "Welcome to zClone! This is a ChatGPT-style interface built with GPUI.".to_string()
            ));
            session.messages.push(Message::new(
                Role::Assistant,
                "Hello! I'm ready to help. How can I assist you today?".to_string()
            ));
            session.preview = Some("Hello! I'm ready to help...".to_string());

            state.active_session_id = Some(session.id.clone());

            // Save to persistence
            if let Some(store) = &mut self.persistence {
                let _ = store.save_session(&session);
            }

            state.sessions.insert(0, session);
        });
    }

    pub fn delete_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        self.app_state.update(cx, |state, _| {
            state.sessions.retain(|s| s.id != session_id);

            // If deleted session was active, select next one
            if state.active_session_id.as_ref() == Some(&session_id) {
                state.active_session_id = state.sessions.first().map(|s| s.id.clone());
            }
        });

        // Delete from persistence
        if let Some(store) = &mut self.persistence {
            let _ = store.delete_session(&session_id);
        }
    }

    pub fn set_active_session(&mut self, session_id: String, cx: &mut Context<Self>) {
        self.app_state.update(cx, |state, _| {
            if state.sessions.iter().any(|s| s.id == session_id) {
                state.active_session_id = Some(session_id);
            }
        });
    }

    pub fn update_composer_text(&mut self, text: String, _cx: &mut Context<Self>) {
        self.composer_text = text;
    }

    pub fn send_message(&mut self, cx: &mut Context<Self>) {
        let text = self.composer_text.trim().to_string();
        if text.is_empty() {
            return;
        }

        let active_id = self.app_state.read(cx).active_session_id.clone();
        if let Some(session_id) = active_id {
            self.app_state.update(cx, |state, _| {
                if let Some(session) = state.sessions.iter_mut().find(|s| s.id == session_id) {
                    // Add user message
                    let user_message = Message::new(Role::User, text.clone());
                    session.messages.push(user_message);

                    // Update preview with latest message
                    session.preview = Some(text.chars().take(60).collect());

                    // Save session to persistence
                    if let Some(store) = &mut self.persistence {
                        let _ = store.save_session(session);
                    }

                    // TODO: In Phase 6, this will trigger OpenAI API call
                    // For now, add a placeholder assistant response
                    let assistant_message = Message::new(
                        Role::Assistant,
                        "This is a placeholder response. OpenAI integration coming in Phase 6!".to_string()
                    );
                    session.messages.push(assistant_message);
                }
            });

            // Clear composer
            self.composer_text.clear();
        }
    }
}

impl Render for AppWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let sessions = self.app_state.read(cx).sessions.clone();
        let active_id = self.app_state.read(cx).active_session_id.clone();

        div()
            .flex()
            .size_full()
            .bg(theme.colors.surface_primary)
            .child(
                // Sidebar
                self.render_sidebar(cx, &sessions, &active_id)
            )
            .child(
                // Main content
                self.render_main_content(cx, &active_id)
            )
    }
}

impl AppWindow {
    fn render_sidebar(
        &mut self,
        cx: &mut Context<Self>,
        sessions: &[ChatSession],
        active_id: &Option<String>,
    ) -> impl IntoElement {
        let theme = &self.theme;

        if self.sidebar_collapsed {
            return div().w(px(0.0));
        }

        div()
            .w(px(280.0))
            .h_full()
            .bg(theme.colors.sidebar_bg)
            .border_r_1()
            .border_color(theme.colors.border_primary)
            .flex()
            .flex_col()
            .child(
                // Header with New Chat button
                div()
                    .h(px(56.0))
                    .px(theme.spacing.md)
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(theme.colors.border_primary)
                    .child(
                        div()
                            .text_color(theme.colors.text_primary)
                            .text_size(theme.typography.display_small.font_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Chats")
                    )
                    .child(
                        div()
                            .id("new-chat-button")
                            .px(theme.spacing.sm)
                            .py(theme.spacing.xs)
                            .bg(theme.colors.accent)
                            .rounded(theme.radius.md)
                            .text_color(theme.colors.text_inverse)
                            .text_size(theme.typography.body_medium.font_size)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                                this.create_new_session(cx);
                            }))
                            .child("New +")
                    )
            )
            .child(
                // Session list
                div()
                    .flex_1()
                    .p(theme.spacing.sm)
                    .flex()
                    .flex_col()
                    .gap(theme.spacing.xs)
                    .children(
                        sessions.iter().map(|session| {
                            self.render_session_item(cx, session, active_id)
                        })
                    )
            )
    }

    fn render_session_item(
        &mut self,
        cx: &mut Context<Self>,
        session: &ChatSession,
        active_id: &Option<String>,
    ) -> impl IntoElement {
        let theme = &self.theme;
        let is_active = active_id.as_ref() == Some(&session.id);
        let session_id = session.id.clone();
        let session_id_for_delete = session.id.clone();

        div()
            .id(SharedString::from(format!("session-item-{}", session_id)))
            .p(theme.spacing.md)
            .rounded(theme.radius.md)
            .cursor_pointer()
            .when(is_active, |div| {
                div.bg(theme.colors.sidebar_item_active)
            })
            .when(!is_active, |div| {
                div.hover(|style| style.bg(theme.colors.sidebar_item_hover))
            })
            .on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                this.set_active_session(session_id.clone(), cx);
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing.xs)
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_color(theme.colors.text_primary)
                                    .text_size(theme.typography.body_medium.font_size)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(session.title.clone())
                            )
                            .child(
                                div()
                                    .id(SharedString::from(format!("delete-session-{}", session_id_for_delete)))
                                    .px(theme.spacing.xs)
                                    .py(px(2.0))
                                    .rounded(theme.radius.sm)
                                    .text_color(theme.colors.error)
                                    .text_size(theme.typography.body_small.font_size)
                                    .cursor_pointer()
                                    .hover(|style| style.bg(theme.colors.surface_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                        this.delete_session(session_id_for_delete.clone(), cx);
                                    }))
                                    .child("✕")
                            )
                    )
                    .child(
                        div()
                            .text_color(theme.colors.text_secondary)
                            .text_size(theme.typography.body_small.font_size)
                            .child(
                                session.preview
                                    .clone()
                                    .unwrap_or_else(|| "No messages yet".to_string())
                            )
                    )
            )
    }

    fn render_main_content(
        &mut self,
        cx: &mut Context<Self>,
        active_id: &Option<String>,
    ) -> impl IntoElement {
        let theme = self.theme.clone();
        let sessions = self.app_state.read(cx).sessions.clone();
        let active_session = active_id.as_ref()
            .and_then(|id| sessions.iter().find(|s| &s.id == id))
            .cloned();
        let sidebar_collapsed = self.sidebar_collapsed;

        let content = if let Some(session) = active_session {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .child(self.render_chat_view_with_composer(cx, &session))
        } else {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex_1()
                        .p(theme.spacing.xxl)
                        .child(self.render_empty_state())
                )
        };

        let title = active_id.as_ref()
            .and_then(|id| sessions.iter().find(|s| &s.id == id))
            .map(|s| s.title.clone())
            .unwrap_or_else(|| "zClone".to_string());

        div()
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .child(
                // Header
                div()
                    .h(px(56.0))
                    .border_b_1()
                    .border_color(theme.colors.border_primary)
                    .flex()
                    .items_center()
                    .px(theme.spacing.lg)
                    .justify_between()
                    .child(
                        div()
                            .text_color(theme.colors.text_primary)
                            .text_size(theme.typography.display_small.font_size)
                            .font_weight(FontWeight::BOLD)
                            .child(title)
                    )
                    .child(
                        div()
                            .id("toggle-sidebar")
                            .px(theme.spacing.sm)
                            .py(theme.spacing.xs)
                            .rounded(theme.radius.md)
                            .text_color(theme.colors.text_secondary)
                            .text_size(theme.typography.body_medium.font_size)
                            .cursor_pointer()
                            .hover(|style| style.bg(theme.colors.surface_hover))
                            .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                                this.toggle_sidebar(cx);
                            }))
                            .child(if sidebar_collapsed { "☰ Show" } else { "☰ Hide" })
                    )
            )
            .child(content)
    }

    fn render_chat_view_with_composer(&mut self, cx: &mut Context<Self>, session: &ChatSession) -> impl IntoElement {
        let theme = &self.theme;

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // Messages area (scrollable)
                div()
                    .flex_1()
                    .p(theme.spacing.xxl)
                    .child(self.render_chat_view(session))
            )
            .child(
                // Composer area (fixed at bottom)
                self.render_composer(cx)
            )
    }

    fn render_chat_view(&self, session: &ChatSession) -> impl IntoElement {
        let theme = &self.theme;
        let is_empty = session.messages.is_empty();

        div()
            .flex()
            .flex_col()
            .gap(theme.spacing.lg)
            .child(
                div()
                    .text_color(theme.colors.text_primary)
                    .text_size(theme.typography.display_medium.font_size)
                    .font_weight(FontWeight::BOLD)
                    .child(session.title.clone())
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing.md)
                    .children(
                        session.messages.iter().map(|msg| {
                            self.render_message(msg)
                        })
                    )
                    .when(is_empty, |d| {
                        d.child(
                            div()
                                .text_color(theme.colors.text_secondary)
                                .text_size(theme.typography.body_large.font_size)
                                .child("No messages yet. Start a conversation!")
                        )
                    })
            )
    }

    fn render_message(&self, message: &Message) -> impl IntoElement {
        let theme = &self.theme;
        let bg_color = match message.role {
            Role::User => theme.colors.message_user,
            Role::Assistant => theme.colors.message_assistant,
            Role::System => theme.colors.message_system,
        };

        div()
            .p(theme.spacing.md)
            .bg(bg_color)
            .rounded(theme.radius.md)
            .border_1()
            .border_color(theme.colors.border_secondary)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing.xs)
                    .child(
                        div()
                            .text_color(theme.colors.text_secondary)
                            .text_size(theme.typography.body_small.font_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(format!("{:?}", message.role))
                    )
                    .child(
                        div()
                            .text_color(theme.colors.text_primary)
                            .text_size(theme.typography.body_medium.font_size)
                            .line_height(theme.typography.body_medium.line_height)
                            .child(message.content.clone())
                    )
            )
    }

    fn render_empty_state(&self) -> impl IntoElement {
        let theme = &self.theme;

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(theme.spacing.lg)
            .child(
                div()
                    .text_color(theme.colors.text_primary)
                    .text_size(theme.typography.display_large.font_size)
                    .font_weight(FontWeight::BOLD)
                    .child("Welcome to zClone")
            )
            .child(
                div()
                    .text_color(theme.colors.text_secondary)
                    .text_size(theme.typography.body_large.font_size)
                    .child("Create a new chat to get started")
            )
    }

    fn render_composer(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let text = self.composer_text.clone();
        let is_empty = text.trim().is_empty();

        div()
            .border_t_1()
            .border_color(theme.colors.border_primary)
            .bg(theme.colors.surface_secondary)
            .p(theme.spacing.md)
            .child(
                div()
                    .flex()
                    .gap(theme.spacing.sm)
                    .items_end()
                    .child(
                        // Text input area
                        div()
                            .flex_1()
                            .min_h(px(48.0))
                            .max_h(px(200.0))
                            .p(theme.spacing.sm)
                            .bg(theme.colors.surface_primary)
                            .border_1()
                            .border_color(theme.colors.border_primary)
                            .rounded(theme.radius.md)
                            .text_color(theme.colors.text_primary)
                            .text_size(theme.typography.body_medium.font_size)
                            .child(
                                // Placeholder for actual text input
                                // GPUI doesn't have a built-in textarea, so we'll use a div for now
                                // In a real implementation, this would be a proper input element
                                div()
                                    .child(
                                        if text.is_empty() {
                                            "Type a message... (Note: This is a visual mock - full input in Phase 6)".to_string()
                                        } else {
                                            text.clone()
                                        }
                                    )
                            )
                    )
                    .child(
                        // Send button
                        div()
                            .id("send-message-button")
                            .px(theme.spacing.md)
                            .py(theme.spacing.sm)
                            .min_w(px(80.0))
                            .h(px(48.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(theme.radius.md)
                            .text_color(if is_empty { theme.colors.text_disabled } else { theme.colors.text_inverse })
                            .text_size(theme.typography.body_medium.font_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .when(!is_empty, |div| {
                                div.bg(theme.colors.accent)
                                    .cursor_pointer()
                                    .hover(|style| style.bg(theme.colors.accent_hover))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                                        this.send_message(cx);
                                    }))
                            })
                            .when(is_empty, |div| {
                                div.bg(theme.colors.surface_tertiary)
                            })
                            .child("Send")
                    )
            )
            .child(
                // Helper text
                div()
                    .mt(theme.spacing.xs)
                    .text_color(theme.colors.text_tertiary)
                    .text_size(theme.typography.body_small.font_size)
                    .child("Cmd+Enter to send · Shift+Enter for new line (coming in Phase 6)")
            )
    }
}
