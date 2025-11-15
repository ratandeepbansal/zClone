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
                    title: Some("zClone - Phase 5: Chat Surface & Composer".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| AppWindow::new(window, cx),
        )
        .unwrap();
        cx.activate(true);
    });
}
