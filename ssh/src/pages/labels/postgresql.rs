use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct PostgreSQL {}

const POSTGRESQL_BG: Color = Color::Rgb(51, 103, 145);

impl PostgreSQL {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("postgresql", WHITE, POSTGRESQL_BG)
    }
}
