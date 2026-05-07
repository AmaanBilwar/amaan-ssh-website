use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Rust {}

const RUST_BG: Color = Color::Rgb(222, 165, 132);

impl Rust {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("rust", WHITE, RUST_BG)
    }
}
