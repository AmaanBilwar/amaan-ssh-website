use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Electron {}

const ELECTRON_BG: Color = Color::Rgb(71, 132, 143);

impl Electron {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("electron", WHITE, ELECTRON_BG)
    }
}
