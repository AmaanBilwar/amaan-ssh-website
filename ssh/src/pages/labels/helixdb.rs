use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct HelixDB {}

const HELIXDB_BG: Color = Color::Rgb(139, 92, 246);

impl HelixDB {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("helixdb", WHITE, HELIXDB_BG)
    }
}
