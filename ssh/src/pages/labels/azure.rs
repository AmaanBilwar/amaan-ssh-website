use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Azure {}

const AZURE_BG: Color = Color::Rgb(0, 137, 214);

impl Azure {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("azure", WHITE, AZURE_BG)
    }
}
