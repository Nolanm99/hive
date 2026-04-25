use gpui::*;
use hive_core::Session;

use crate::theme::{Palette, pixel_cell};

pub fn sidebar(sessions: &[Session], active_session: usize, palette: &Palette) -> impl IntoElement {
    div()
        .w(px(240.0))
        .h_full()
        .pb_8()
        .bg(palette.panel)
        .border_r_1()
        .border_color(palette.border_dark)
        .child(
            div()
                .h(px(58.0))
                .px_3()
                .flex()
                .items_center()
                .gap_2()
                .border_b_1()
                .border_color(palette.border_dark)
                .bg(palette.panel_alt)
                .child(
                    div()
                        .grid()
                        .grid_cols(3)
                        .gap_1()
                        .child(pixel_cell(10.0, palette.cell, palette.border_dark))
                        .child(pixel_cell(10.0, palette.accent, palette.border_dark))
                        .child(pixel_cell(10.0, palette.cell_dim, palette.border_dark))
                        .child(pixel_cell(10.0, palette.cell_dim, palette.border_dark))
                        .child(pixel_cell(10.0, palette.cell, palette.border_dark))
                        .child(pixel_cell(10.0, palette.accent_alt, palette.border_dark)),
                )
                .child(
                    div()
                        .text_size(px(18.0))
                        .text_color(palette.accent)
                        .child("HIVE"),
                ),
        )
        .child(section_title("Sessions", palette))
        .children(sessions.iter().enumerate().map(|(ix, session)| {
            let selected = ix == active_session;
            div()
                .mx_2()
                .mb_1()
                .px_2()
                .py_2()
                .rounded(px(0.0))
                .border_1()
                .border_color(if selected {
                    palette.accent
                } else {
                    palette.border_dark
                })
                .bg(if selected {
                    palette.cell_dim
                } else {
                    palette.panel
                })
                .text_color(if selected {
                    palette.foreground
                } else {
                    palette.muted
                })
                .child(session.name.clone())
        }))
        .child(section_title("Worktrees", palette))
        .child(list_item("main", palette))
        .child(list_item("feat/orchestration-ui", palette))
}

fn section_title(title: &str, palette: &Palette) -> impl IntoElement {
    div()
        .px_3()
        .pt_4()
        .pb_2()
        .text_size(px(11.0))
        .text_color(palette.accent)
        .child(title.to_uppercase())
}

fn list_item(label: &str, palette: &Palette) -> impl IntoElement {
    div()
        .mx_2()
        .mb_1()
        .px_2()
        .py_2()
        .border_1()
        .border_color(palette.border_dark)
        .bg(palette.background)
        .text_size(px(13.0))
        .text_color(palette.foreground)
        .child(label.to_string())
}
