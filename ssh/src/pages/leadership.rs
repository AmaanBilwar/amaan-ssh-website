use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Constraint,
    layout::Rect,
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::page::Page;
use crate::pages::style::{
    dimmed_selected_style, gray_span, gray_style, line_from_spans, selected_style, white_span,
};

struct ExperienceItem {
    role: &'static str,
    affiliation: &'static str,
    time: &'static str,
    description: Vec<&'static str>,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&str; 3] {
        [self.role, self.affiliation, self.time]
    }
}

pub struct Leadership {
    state: usize,
    experiences: Vec<ExperienceItem>,
}

impl Leadership {
    pub fn new() -> Self {
        let experiences = vec![
            ExperienceItem {
                role: "founder & ceo",
                affiliation: "scene ai",
                time: "(april 2025-present)",
                description: vec![
                    "building ai-powered solutions for scene understanding and computer vision applications",
                ],
            },
            ExperienceItem {
                role: "founder & ceo",
                affiliation: "soar ai labs",
                time: "(sept 2025-present)",
                description: vec![
                    "leading ai research and development lab focused on cutting-edge machine learning solutions",
                ],
            },
            ExperienceItem {
                role: "web lead",
                affiliation: "revolutionuc",
                time: "(may 2025-april 2025)",
                description: vec![
                    "leading the web development team for cincinnati's premier hackathon",
                    "",
                    "notable highlights:",
                    "- managing website and platform infrastructure for 200+ participants",
                ],
            },
            ExperienceItem {
                role: "daq creator & lead",
                affiliation: "bearcats electric racing",
                time: "(aug 2024-may 2025)",
                description: vec![
                    "created and led the data acquisition system for the ev3 formula electric vehicle",
                    "",
                    "notable highlights:",
                    "- designed custom data acquisition architecture for vehicle telemetry",
                    "- 2025 season competition team",
                ]
            }
        ];

        Self {
            state: 0,
            experiences,
        }
    }

    fn previous_experience(&mut self) {
        if self.state > 0 {
            self.state -= 1;
        }
    }

    fn next_experience(&mut self) {
        if self.state < self.experiences.len() - 1 {
            self.state += 1;
        }
    }

    fn get_description(&self) -> Vec<Line<'_>> {
        let experience_index = self.state;
        let mut final_vec: Vec<Line<'_>> = vec![];
        let experience_item = &self.experiences[experience_index];

        for desc_part in &experience_item.description {
            final_vec.push(line_from_spans(vec![gray_span(&desc_part)]));
        }

        final_vec
    }
}

impl Page for Leadership {
    fn title(&self) -> &str {
        "leadership"
    }

    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let header = ["role", "affiliation", "time"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.experiences.iter().enumerate().map(|(i, data)| {
            let item = data.ref_array();

            let style_config = match i == self.state {
                true => {
                    if is_focused {
                        selected_style()
                    } else {
                        dimmed_selected_style()
                    }
                }
                false => gray_style(),
            };

            item.into_iter()
                .map(|content| Cell::from(content))
                .collect::<Row>()
                .style(style_config)
                .height(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Fill(2),
                Constraint::Length(19),
            ],
        )
        .header(header)
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 1,
            bottom: 0,
        }));

        frame.render_widget(table, area);
    }

    fn render_additional(&self, frame: &mut Frame, area: Rect, _is_focused: bool) {
        let mut description = self.get_description();
        description.insert(0, line_from_spans(vec![white_span("desc")]));

        let paragraph = Paragraph::new(description).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous_experience();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next_experience();
            }
            _ => {}
        }
    }
}
