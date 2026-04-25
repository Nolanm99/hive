use gpui::*;
use gpui_component::scroll::ScrollableElement;
use hive_core::Session;

use crate::{input_bar::input_bar, message::message_view, theme::Palette};

pub fn chat_view(session: &Session, input_text: &str, palette: &Palette) -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .pb_8()
        .flex()
        .flex_col()
        .bg(palette.background)
        .child(
            div()
                .flex_1()
                .overflow_y_scrollbar()
                .px_5()
                .py_4()
                .border_l_1()
                .border_r_1()
                .border_color(palette.border_dark)
                .children(
                    session
                        .messages
                        .iter()
                        .map(|message| message_view(message, palette)),
                ),
        )
        .child(input_bar(input_text, palette))
}
