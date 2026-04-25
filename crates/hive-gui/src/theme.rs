use gpui::*;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Palette {
    pub background: Hsla,
    pub panel: Hsla,
    pub panel_alt: Hsla,
    pub foreground: Hsla,
    pub muted: Hsla,
    pub border: Hsla,
    pub border_dark: Hsla,
    pub accent: Hsla,
    pub accent_alt: Hsla,
    pub cell: Hsla,
    pub cell_dim: Hsla,
    pub positive: Hsla,
    pub negative: Hsla,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            background: hsla(43.0 / 360.0, 0.46, 0.09, 1.0),
            panel: hsla(39.0 / 360.0, 0.44, 0.13, 1.0),
            panel_alt: hsla(42.0 / 360.0, 0.54, 0.18, 1.0),
            foreground: hsla(48.0 / 360.0, 0.72, 0.88, 1.0),
            muted: hsla(47.0 / 360.0, 0.30, 0.58, 1.0),
            border: hsla(46.0 / 360.0, 0.90, 0.38, 1.0),
            border_dark: hsla(28.0 / 360.0, 0.48, 0.06, 1.0),
            accent: hsla(46.0 / 360.0, 0.95, 0.55, 1.0),
            accent_alt: hsla(188.0 / 360.0, 0.42, 0.42, 1.0),
            cell: hsla(44.0 / 360.0, 0.92, 0.46, 1.0),
            cell_dim: hsla(36.0 / 360.0, 0.60, 0.24, 1.0),
            positive: hsla(145.0 / 360.0, 0.46, 0.48, 1.0),
            negative: hsla(4.0 / 360.0, 0.58, 0.56, 1.0),
        }
    }
}

pub fn pixel_cell(size: f32, color: Hsla, border: Hsla) -> impl IntoElement {
    div()
        .size(px(size))
        .bg(color)
        .border_1()
        .border_color(border)
}
