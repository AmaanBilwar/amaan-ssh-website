use crate::pages::{labels::label::ColoredLabel, style::BLACK};
use ratatui::style::Color;

pub struct Bun {}

const BUN_BG: Color = Color::Rgb(251, 240, 223);

impl Bun {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("bun", BLACK, BUN_BG)
    }
}
