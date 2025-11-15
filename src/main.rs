use gpui::*;
use zclone_ui::*;

fn main() {
    env_logger::init();

    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("zClone - Phase 3 Complete".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ZCloneApp::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}

struct ZCloneApp {
    theme: ZCloneTheme,
}

impl ZCloneApp {
    fn new() -> Self {
        Self {
            theme: ZCloneTheme::dark(),
        }
    }
}

impl Render for ZCloneApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;

        div()
            .flex()
            .size_full()
            .bg(theme.colors.surface_primary)
            .child(
                // Sidebar demo
                div()
                    .w(px(280.0))
                    .h_full()
                    .bg(theme.colors.sidebar_bg)
                    .border_r_1()
                    .border_color(theme.colors.border_primary)
                    .flex()
                    .flex_col()
                    .p(theme.spacing.md)
                    .child(
                        div()
                            .text_color(theme.colors.text_primary)
                            .text_size(theme.typography.display_small.font_size)
                            .font_weight(FontWeight::SEMIBOLD)
                            .mb(theme.spacing.lg)
                            .child("Chat Sessions")
                    )
                    .child(
                        // Session item example
                        div()
                            .p(theme.spacing.md)
                            .bg(theme.colors.sidebar_item_active)
                            .rounded(theme.radius.md)
                            .mb(theme.spacing.xs)
                            .child(
                                div()
                                    .text_color(theme.colors.text_primary)
                                    .text_size(theme.typography.body_medium.font_size)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("Active Chat")
                            )
                            .child(
                                div()
                                    .text_color(theme.colors.text_secondary)
                                    .text_size(theme.typography.body_small.font_size)
                                    .child("Design system implementation...")
                            )
                    )
            )
            .child(
                // Main content area
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
                            .child(
                                div()
                                    .text_color(theme.colors.text_primary)
                                    .text_size(theme.typography.display_small.font_size)
                                    .font_weight(FontWeight::BOLD)
                                    .child("zClone - Phase 3 Complete")
                            )
                    )
                    .child(
                        // Content
                        div()
                            .flex_1()
                            .p(theme.spacing.xxl)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(theme.spacing.lg)
                                    .child(
                                        div()
                                            .text_color(theme.colors.text_primary)
                                            .text_size(theme.typography.display_medium.font_size)
                                            .font_weight(FontWeight::BOLD)
                                            .child("UI/UX System Design Complete")
                                    )
                                    .child(
                                        div()
                                            .text_color(theme.colors.text_secondary)
                                            .text_size(theme.typography.body_large.font_size)
                                            .line_height(theme.typography.body_large.line_height)
                                            .child("Phase 3 has established a comprehensive design system including:")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(theme.spacing.md)
                                            .child(self.render_feature_card(theme, "Typography", "SF Pro Display, SF Pro Text, and SF Mono with 8 size scales"))
                                            .child(self.render_feature_card(theme, "Color System", "24 semantic tokens with WCAG AAA contrast (7:1+)"))
                                            .child(self.render_feature_card(theme, "Spacing & Layout", "8px-based scale from 4px to 64px"))
                                            .child(self.render_feature_card(theme, "Component Specs", "13 detailed component specifications"))
                                            .child(self.render_feature_card(theme, "Motion System", "Animation durations, easing curves, and interactions"))
                                            .child(self.render_feature_card(theme, "Accessibility", "44px touch targets, keyboard shortcuts, reduced motion"))
                                    )
                                    .child(
                                        div()
                                            .mt(theme.spacing.xl)
                                            .p(theme.spacing.lg)
                                            .bg(theme.colors.message_assistant)
                                            .rounded(theme.radius.lg)
                                            .border_1()
                                            .border_color(theme.colors.border_primary)
                                            .child(
                                                div()
                                                    .text_color(theme.colors.text_primary)
                                                    .text_size(theme.typography.body_medium.font_size)
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .mb(theme.spacing.sm)
                                                    .child("Next: Phase 4 - Sidebar & Session Management")
                                            )
                                            .child(
                                                div()
                                                    .text_color(theme.colors.text_secondary)
                                                    .text_size(theme.typography.body_medium.font_size)
                                                    .line_height(theme.typography.body_medium.line_height)
                                                    .child("Implement the sidebar component, session list, and persistence integration")
                                            )
                                    )
                            )
                    )
            )
    }
}

impl ZCloneApp {
    fn render_feature_card(&self, theme: &ZCloneTheme, title: &'static str, description: &'static str) -> impl IntoElement {
        div()
            .p(theme.spacing.md)
            .bg(theme.colors.surface_secondary)
            .rounded(theme.radius.md)
            .border_1()
            .border_color(theme.colors.border_secondary)
            .child(
                div()
                    .text_color(theme.colors.text_primary)
                    .text_size(theme.typography.body_large.font_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .mb(theme.spacing.xs)
                    .child(title)
            )
            .child(
                div()
                    .text_color(theme.colors.text_secondary)
                    .text_size(theme.typography.body_medium.font_size)
                    .child(description)
            )
    }
}
