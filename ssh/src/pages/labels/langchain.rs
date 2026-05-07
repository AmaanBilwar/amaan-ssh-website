use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct LangChain {}

const LANGCHAIN_BG: Color = Color::Rgb(31, 41, 55);

impl LangChain {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("langchain", WHITE, LANGCHAIN_BG)
    }
}
