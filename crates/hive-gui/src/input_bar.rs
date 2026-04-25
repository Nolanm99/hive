use gpui::*;

use crate::theme::{Palette, pixel_cell};

pub fn input_bar(input_text: &str, palette: &Palette) -> impl IntoElement {
    div()
        .mx_5()
        .mb_4()
        .border_1()
        .border_color(palette.border_dark)
        .rounded(px(0.0))
        .bg(palette.panel)
        .child(
            div()
                .px_3()
                .py_2()
                .flex()
                .gap_2()
                .border_b_1()
                .border_color(palette.border_dark)
                .text_size(px(12.0))
                .text_color(palette.muted)
                .child(pixel_cell(12.0, palette.cell, palette.border_dark))
                .child(pixel_cell(12.0, palette.cell_dim, palette.border_dark))
                .child(toolbar_chip("main", palette))
                .child(toolbar_chip("default model", palette))
                .child(toolbar_chip("Codex", palette)),
        )
        .child(
            div()
                .min_h(px(86.0))
                .px_3()
                .py_3()
                .text_size(px(14.0))
                .text_color(if input_text.is_empty() {
                    palette.muted
                } else {
                    palette.foreground
                })
                .child(if input_text.is_empty() {
                    "Type your message...".to_string()
                } else {
                    input_text.to_string()
                }),
        )
        .child(
            div().px_3().pb_3().flex().justify_end().child(
                div()
                    .px_3()
                    .py_2()
                    .rounded(px(0.0))
                    .border_1()
                    .border_color(palette.border_dark)
                    .bg(palette.accent)
                    .text_color(palette.background)
                    .child("Send"),
            ),
        )
}

fn toolbar_chip(label: &str, palette: &Palette) -> impl IntoElement {
    div()
        .px_2()
        .py_1()
        .rounded(px(0.0))
        .border_1()
        .border_color(palette.border_dark)
        .bg(palette.cell_dim)
        .text_color(palette.foreground)
        .child(label.to_string())
}
