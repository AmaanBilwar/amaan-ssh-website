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

fn osc52(text: &str) {
    use base64::{Engine as _, engine::general_purpose};

    let encoded = general_purpose::STANDARD.encode(text.as_bytes());
    print!("\x1b]52;c;{}\x07", encoded);
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
}

struct OssItem {
    project: &'static str,
    contribution: &'static str,
    link: &'static str,
    description: Vec<&'static str>,
}

impl OssItem {
    pub const fn ref_array(&self) -> [&str; 2] {
        [self.project, self.contribution]
    }
}

pub struct Oss {
    state: usize,
    current_link: String,
    contributions: Vec<OssItem>,
    show_tooltip: bool,
    tooltip_end_tick: u64,
    current_tick: u64,
}

impl Page for Oss {
    fn title(&self) -> &str {
        "oss"
    }

    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let [tooltip_area, content_area] =
            ratatui::layout::Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);

        if self.show_tooltip {
            let tooltip_text = "✔ link copied to clipboard";
            let tooltip_paragraph = Paragraph::new(tooltip_text)
                .style(ratatui::style::Style::new().fg(ratatui::style::Color::Green))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(tooltip_paragraph, tooltip_area);
        }

        let header = ["project", "role"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.contributions.iter().enumerate().map(|(i, data)| {
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
            [Constraint::Fill(1), Constraint::Length(15)],
        )
        .header(header)
        .block(Block::new().padding(Padding {
            left: 1,
            right: 2,
            top: 0,
            bottom: 0,
        }));

        frame.render_widget(table, content_area);
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
                self.previous_contribution();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next_contribution();
            }
            KeyCode::Enter => {
                osc52(&self.current_link);
                self.show_tooltip = true;
                self.tooltip_end_tick = self.current_tick + 38;
            }
            _ => {}
        }
    }

    fn nav_items(&self) -> Vec<Line<'static>> {
        vec![line_from_spans(vec![white_span(" ↵  "), gray_span("copy")])]
    }

    fn on_tick(&mut self, tick: u64) -> bool {
        self.current_tick = tick;
        if self.show_tooltip && tick >= self.tooltip_end_tick {
            self.show_tooltip = false;
        }
        true
    }
}

impl Oss {
    pub fn new() -> Self {
        let contributions = vec![
            OssItem {
                project: "zed",
                contribution: "bug basher",
                link: "https://github.com/zed-industries/zed/pulls?q=is%3Apr+author%3AAmaanBilwar",
                description: vec![
                    "active contributor to zed editor - the high-performance, multiplayer code editor",
                    "",
                    "fixing bugs and improving stability across the rust codebase",
                    "",
                    "part of the zed guild - contributors recognized for consistent quality contributions",
                ],
            },
            OssItem {
                project: "helixdb",
                contribution: "core contributor",
                link: "https://github.com/HelixDB/helix-db",
                description: vec![
                    "contributing to helixdb - a graph database designed for ai workloads",
                    "",
                    "working on secret projects involving graph-based data structures",
                    "",
                    "helping build the future of ai-native databases",
                ],
            },
        ];

        Self {
            state: 0,
            current_link: String::from(contributions[0].link),
            contributions,
            show_tooltip: false,
            tooltip_end_tick: 0,
            current_tick: 0,
        }
    }

    fn get_description(&self) -> Vec<Line<'_>> {
        let index = self.state;
        let item = &self.contributions[index];
        let mut final_vec: Vec<Line<'_>> = vec![];

        for desc_part in &item.description {
            final_vec.push(line_from_spans(vec![gray_span(desc_part)]));
        }

        final_vec
    }

    fn previous_contribution(&mut self) {
        if self.state > 0 {
            self.state -= 1;
            self.change_current_link();
        }
    }

    fn next_contribution(&mut self) {
        if self.state < self.contributions.len() - 1 {
            self.state += 1;
            self.change_current_link();
        }
    }

    fn change_current_link(&mut self) {
        self.current_link = String::from(self.contributions[self.state].link);
    }
}
