use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct SpringBoot {}

const SPRING_BOOT_BG: Color = Color::Rgb(109, 179, 63);

impl SpringBoot {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("spring boot", WHITE, SPRING_BOOT_BG)
    }
}
