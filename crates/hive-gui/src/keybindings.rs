use gpui::*;

actions!(hive, [ToggleSidebar, ToggleDiffPanel]);

pub fn register(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-b", ToggleSidebar, None),
        KeyBinding::new("ctrl-d", ToggleDiffPanel, None),
    ]);
}
