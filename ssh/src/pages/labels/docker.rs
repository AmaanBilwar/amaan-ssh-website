use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Docker {}

const DOCKER_BG: Color = Color::Rgb(13, 102, 170);

impl Docker {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("docker", WHITE, DOCKER_BG)
    }
}
