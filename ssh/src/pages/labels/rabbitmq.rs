use crate::pages::{labels::label::ColoredLabel, style::WHITE};
use ratatui::style::Color;

pub struct RabbitMQ {}

const RABBITMQ_BG: Color = Color::Rgb(255, 102, 51);

impl RabbitMQ {
    pub fn build() -> ColoredLabel {
        ColoredLabel::new("rabbitmq", WHITE, RABBITMQ_BG)
    }
}
