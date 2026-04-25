use gpui::*;

use crate::theme::{Palette, pixel_cell};

pub fn diff_panel(palette: &Palette) -> impl IntoElement {
    div()
        .w(px(300.0))
        .h_full()
        .pb_8()
        .bg(palette.panel)
        .border_l_1()
        .border_color(palette.border_dark)
        .child(
            div()
                .px_3()
                .pt_4()
                .pb_2()
                .flex()
                .items_center()
                .gap_2()
                .text_size(px(11.0))
                .text_color(palette.accent)
                .child(pixel_cell(10.0, palette.cell, palette.border_dark))
                .child("CHANGED FILES"),
        )
        .child(file_row("crates/hive-gui/src/app.rs", palette))
        .child(file_row("crates/hive-core/src/agent.rs", palette))
        .child(
            div()
                .mx_3()
                .mt_4()
                .p_2()
                .rounded(px(0.0))
                .border_1()
                .border_color(palette.border_dark)
                .bg(palette.background)
                .font_family("JetBrains Mono")
                .text_size(px(12.0))
                .text_color(palette.foreground)
                .child("- placeholder diff\n+ live git2 diff lands in Phase 6"),
        )
}

fn file_row(label: &str, palette: &Palette) -> impl IntoElement {
    div()
        .mx_2()
        .mb_1()
        .px_2()
        .py_2()
        .border_1()
        .border_color(palette.border_dark)
        .bg(palette.panel_alt)
        .text_size(px(12.0))
        .text_color(palette.foreground)
        .child(label.to_string())
}
