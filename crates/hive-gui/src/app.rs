use std::path::PathBuf;

use gpui::*;
use hive_core::{AgentConfig, Message, MessageRole, Session};

use crate::{
    chat_view::chat_view,
    diff_panel::diff_panel,
    keybindings::{ToggleDiffPanel, ToggleSidebar},
    sidebar::sidebar,
    theme::{self, pixel_cell},
};

pub struct HiveApp {
    sidebar_visible: bool,
    diff_visible: bool,
    sessions: Vec<Session>,
    active_session: usize,
    input_text: String,
}

impl HiveApp {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let mut session = Session::new(
            "Session 1",
            AgentConfig::codex(),
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        );
        session.messages = vec![
            Message::new(
                MessageRole::Agent,
                "# Hive\nReady to orchestrate CLI agents from a structured chat UI.",
            ),
            Message::new(
                MessageRole::User,
                "Scaffold the workspace and show the first three-pane shell.",
            ),
        ];

        Self {
            sidebar_visible: true,
            diff_visible: true,
            sessions: vec![session],
            active_session: 0,
            input_text: String::new(),
        }
    }

    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_visible = !self.sidebar_visible;
        cx.notify();
    }

    fn toggle_diff_panel(&mut self, cx: &mut Context<Self>) {
        self.diff_visible = !self.diff_visible;
        cx.notify();
    }
}

impl Render for HiveApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = theme::Palette::default();
        let active_session = &self.sessions[self.active_session];

        div()
            .id("hive-root")
            .size_full()
            .bg(palette.background)
            .text_color(palette.foreground)
            .font_family("JetBrains Mono")
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| this.toggle_sidebar(cx)))
            .on_action(
                cx.listener(|this, _: &ToggleDiffPanel, _window, cx| this.toggle_diff_panel(cx)),
            )
            .child(
                div()
                    .flex()
                    .h_full()
                    .w_full()
                    .child(if self.sidebar_visible {
                        sidebar(&self.sessions, self.active_session, &palette).into_any_element()
                    } else {
                        div().w_0().into_any_element()
                    })
                    .child(chat_view(active_session, &self.input_text, &palette))
                    .child(if self.diff_visible {
                        diff_panel(&palette).into_any_element()
                    } else {
                        div().w_0().into_any_element()
                    }),
            )
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .h_8()
                    .px_3()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_t_1()
                    .border_color(palette.border_dark)
                    .bg(palette.panel)
                    .text_size(px(12.0))
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .child(pixel_cell(10.0, palette.cell, palette.border_dark))
                            .child(pixel_cell(10.0, palette.cell_dim, palette.border_dark))
                            .child(pixel_cell(10.0, palette.accent_alt, palette.border_dark)),
                    )
                    .child(format!(
                        "{} · {}",
                        active_session.name,
                        active_session.working_dir.display()
                    ))
                    .child("Ctrl+B sidebar · Ctrl+D diff"),
            )
    }
}
