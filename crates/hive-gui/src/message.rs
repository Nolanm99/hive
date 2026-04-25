use gpui::prelude::FluentBuilder;
use gpui::*;
use hive_core::{Message, MessageRole};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

use crate::theme::Palette;

pub fn message_view(message: &Message, palette: &Palette) -> impl IntoElement {
    let is_user = message.role == MessageRole::User;
    div()
        .mb_4()
        .flex()
        .justify_end()
        .when(!is_user, |this| this.justify_start())
        .child(
            div()
                .max_w(px(760.0))
                .p_3()
                .rounded(px(0.0))
                .bg(if is_user {
                    palette.cell_dim
                } else {
                    palette.panel
                })
                .border_1()
                .border_color(if is_user {
                    palette.accent
                } else {
                    palette.border_dark
                })
                .children(markdown_blocks(&message.content, palette)),
        )
}

fn markdown_blocks(markdown: &str, palette: &Palette) -> Vec<AnyElement> {
    let mut blocks = Vec::new();
    let mut text = String::new();
    let mut in_code_block = false;
    let mut code_language = String::new();

    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::Heading { .. }) => text.push_str("\n"),
            Event::Start(Tag::CodeBlock(kind)) => {
                flush_text(&mut blocks, &mut text, palette);
                in_code_block = true;
                code_language = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                blocks.push(code_block(&code_language, &text, palette).into_any_element());
                text.clear();
                in_code_block = false;
                code_language.clear();
            }
            Event::End(TagEnd::Paragraph | TagEnd::Heading(_)) if !in_code_block => {
                text.push('\n');
                flush_text(&mut blocks, &mut text, palette);
            }
            Event::Text(value) | Event::Code(value) => text.push_str(&value),
            Event::SoftBreak | Event::HardBreak => text.push('\n'),
            _ => {}
        }
    }

    flush_text(&mut blocks, &mut text, palette);
    blocks
}

fn flush_text(blocks: &mut Vec<AnyElement>, text: &mut String, palette: &Palette) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        text.clear();
        return;
    }

    blocks.push(
        div()
            .mb_2()
            .text_size(px(14.0))
            .line_height(px(21.0))
            .text_color(palette.foreground)
            .child(trimmed.to_string())
            .into_any_element(),
    );
    text.clear();
}

fn code_block(language: &str, code: &str, palette: &Palette) -> impl IntoElement {
    div()
        .my_2()
        .border_1()
        .border_color(palette.border_dark)
        .rounded(px(0.0))
        .overflow_hidden()
        .child(
            div()
                .px_2()
                .py_1()
                .bg(palette.cell_dim)
                .text_size(px(11.0))
                .text_color(palette.accent)
                .child(
                    if language.is_empty() {
                        "text"
                    } else {
                        language
                    }
                    .to_string(),
                ),
        )
        .child(
            div()
                .p_2()
                .bg(palette.background)
                .font_family("JetBrains Mono")
                .text_size(px(13.0))
                .line_height(px(19.0))
                .child(code.to_string()),
        )
}
