use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct Hazelcast {}

const HAZELCAST_BG: Color = Color::Rgb(13, 88, 139);

impl Hazelcast {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("hazelcast", WHITE, HAZELCAST_BG)
    }
}
