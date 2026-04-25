mod app;
mod chat_view;
mod command_palette;
mod diff_panel;
mod input_bar;
mod keybindings;
mod message;
mod sidebar;
mod theme;
mod workspace;

use gpui::*;

use crate::app::HiveApp;

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);
        keybindings::register(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| HiveApp::new(window, cx));
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })
            .expect("failed to open Hive window");
        })
        .detach();
    });
}
