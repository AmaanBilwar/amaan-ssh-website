use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, Wrap},
};
use std::env;

use crate::pages::{
    labels::{
        azure::Azure,
        docker::Docker,
        hazelcast::Hazelcast,
        label::ColoredLabel,
        langchain::LangChain,
        postgresql::PostgreSQL,
        python::Python,
        rabbitmq::RabbitMQ,
        spring_boot::SpringBoot,
    },
    style::{
        WHITE, dimmed_selected_style, gray_span, gray_style, line_from_spans, selected_style,
        white_span,
    },
};
use crate::pages::{
    labels::{container::LabelContainer},
    page::Page,
};

struct ExperienceItem {
    role: &'static str,
    affiliation: &'static str,
    time: &'static str,
    description: Vec<&'static str>,
    technologies: Vec<ColoredLabel>,
}

impl ExperienceItem {
    pub const fn ref_array(&self) -> [&str; 3] {
        [self.role, self.affiliation, self.time]
    }
}

pub struct Experience {
    state: usize,
    experiences: Vec<ExperienceItem>,
    show_tech_block: bool,
}

impl Experience {
    pub fn new() -> Self {
        let show_widgets = env::var("SHOW_WIDGETS").unwrap_or_default();
        let show_tech_block = show_widgets == "TECH" || show_widgets == "ALL";

        let experiences = vec![
            ExperienceItem {
                role: "ai engineer intern",
                affiliation: "story",
                time: "(sep 2025-dec 2025)",
                description: vec![
                    "shipped an ios ai video backend in 1 week for prompt-to-video generation (script, characters, music, render)",
                    "",
                    "notable highlights:",
                    "- built a custom music agent with json-rpc using acp for scene-aware soundtrack generation",
                    "- implemented multi-agent communication workflows to speed script/character/media generation and reduce time-to-video",
                ],
                technologies: vec![Python::build(), LangChain::build()],
            },
            ExperienceItem {
                role: "swe intern",
                affiliation: "honeywell",
                time: "(sep 2024-dec 2024)",
                description: vec![
                    "built an log-search platform processing 90tb+ telemetry/day; cut error detection time by 70%",
                    "",
                    "notable highlights:",
                    "- integrated semantic search and anomaly triage into cloud microservices with sre/platform teams",
                    "- delivered the end-to-end stack for low-latency incident search and faster root-cause analysis",
                ],
                technologies: vec![Python::build(), LangChain::build(), Azure::build()],
            },
            ExperienceItem {
                role: "swe intern",
                affiliation: "honeywell",
                time: "(jan 2024-apr 2024)",
                description: vec![
                    "reduced customer onboarding time by 64% by redesigning workflows and backend architecture",
                    "",
                    "notable highlights:",
                    "- improved reliability/fault tolerance via spring cloud azure, rabbitmq, and hazelcast",
                ],
                technologies: vec![
                    Docker::build(),
                    PostgreSQL::build(),
                    SpringBoot::build(),
                    RabbitMQ::build(),
                    Hazelcast::build(),
                ],
            },
        ];

        Self {
            state: 0,
            experiences,
            show_tech_block,
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

impl Page for Experience {
    fn title(&self) -> &str {
        "experience"
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
                Constraint::Fill(1),
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

        let text_height = paragraph.line_count(area.width); // NOTE: this feature is experimental and potentially unstable
        let available_height = area.height;

        // Ensure we don't exceed available height and reserve space for tech block
        let actual_text_height = (text_height as u16).min(available_height.saturating_sub(4));

        let [text_area, tech_area] =
            Layout::vertical([Constraint::Length(actual_text_height), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        frame.render_widget(paragraph, text_area);

        if self.show_tech_block {
            let tech_block = Block::new()
                .title("tech")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(WHITE))
                .padding(Padding {
                    left: 1,
                    right: 1,
                    top: 0,
                    bottom: 0,
                });

            frame.render_widget(tech_block, tech_area);
        }

        let experience_item = &self.experiences[self.state];
        let container = LabelContainer::new(&experience_item.technologies);
        container.render(frame, tech_area);
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
