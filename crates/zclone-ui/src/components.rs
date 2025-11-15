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
            }
        })
    }

    pub fn toggle_sidebar(&mut self, _cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
    }

    pub fn create_new_session(&mut self, cx: &mut Context<Self>) {
        self.app_state.update(cx, |state, _| {
            let session = ChatSession::new("New Chat");
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
        let theme = &self.theme;
        let sessions = self.app_state.read(cx).sessions.clone();
        let active_session = active_id.as_ref()
            .and_then(|id| sessions.iter().find(|s| &s.id == id));

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
                            .child(
                                active_session
                                    .map(|s| s.title.clone())
                                    .unwrap_or_else(|| "zClone".to_string())
                            )
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
                            .child(if self.sidebar_collapsed { "☰ Show" } else { "☰ Hide" })
                    )
            )
            .child(
                // Content area
                div()
                    .flex_1()
                    .p(theme.spacing.xxl)
                    .when_some(active_session, |div, session| {
                        div.child(self.render_chat_view(session))
                    })
                    .when(active_session.is_none(), |div| {
                        div.child(self.render_empty_state())
                    })
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
}
