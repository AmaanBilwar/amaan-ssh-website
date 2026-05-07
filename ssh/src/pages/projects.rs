use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Table, Wrap},
};

use crate::pages::{
    labels::{
        bun::Bun,
        container::LabelContainer,
        electron::Electron,
        helixdb::HelixDB,
        javascript::JavaScript,
        label::ColoredLabel,
        rust::Rust,
        typescript::TypeScript,
    },
    page::Page,
    style::{
        dimmed_selected_style, gray_span, gray_style, line_from_spans, selected_style, white_span,
    },
};

fn osc52(text: &str) {
    use base64::{Engine as _, engine::general_purpose};

    let encoded = general_purpose::STANDARD.encode(text.as_bytes());
    print!("\x1b]52;c;{}\x07", encoded);
    // Flush to ensure the sequence is sent immediately
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
}
struct ProjectItem {
    name: &'static str,
    link: &'static str,
    project_type: &'static str,
    prizes: Vec<&'static str>,
    description: Vec<&'static str>,
    technologies: Vec<ColoredLabel>,
}

impl ProjectItem {
    pub const fn ref_name(&self) -> [&str; 2] {
        [self.name, self.project_type]
    }
}

pub struct Projects {
    state: usize,
    current_link: String,
    projects: Vec<ProjectItem>,
    show_tooltip: bool,
    tooltip_end_tick: u64,
    current_tick: u64,
}

impl Page for Projects {
    fn title(&self) -> &str {
        "projects"
    }

    fn render(&self, frame: &mut Frame, area: Rect, is_focused: bool) {
        // Split area into tooltip area and content area
        let [tooltip_area, content_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);

        // Render tooltip if active
        if self.show_tooltip {
            let tooltip_text = "✔ project link copied to clipboard";
            let tooltip_paragraph = Paragraph::new(tooltip_text)
                .style(ratatui::style::Style::new().fg(ratatui::style::Color::Green))
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(tooltip_paragraph, tooltip_area);
        }

        let header = ["name", "project type"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.projects.iter().enumerate().map(|(i, data)| {
            let item = data.ref_name();

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

        // Calculate max width needed for project type column
        let max_project_type_len = self
            .projects
            .iter()
            .map(|p| p.project_type.len())
            .max()
            .unwrap_or(0) as u16;

        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Length(max_project_type_len),
            ],
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

        let text_height = paragraph.line_count(area.width); // NOTE: this feature is experimental and potentially unstable
        let available_height = area.height;

        // Ensure we don't exceed available height and reserve space for tech block
        let actual_text_height = (text_height as u16).min(available_height.saturating_sub(4));

        let [text_area, tech_area] =
            Layout::vertical([Constraint::Length(actual_text_height), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);

        frame.render_widget(paragraph, text_area);

        let project_item = &self.projects[self.state];
        let container = LabelContainer::new(&project_item.technologies);
        container.render(frame, tech_area);
    }

    fn keyboard_event_handler(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous_project();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next_project();
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

impl Projects {
    pub fn new() -> Self {
        let projects = vec![
            ProjectItem {
                name: "the-search-thing",
                link: "https://github.com/amaanbilwar/the-search-thing",
                prizes: vec![],
                description: vec![
                    "a local-first semantic search app across files, images, and videos with sub-ms response targets",
                    "",
                    "architected an electron ui with a rust sidecar over json-rpc (ndjson over stdio) to deliver stable, low-latency app-to-core communication",
                ],
                project_type: "personal",
                technologies: vec![
                    Rust::build(),
                    Electron::build(),
                    HelixDB::build(),
                ],
            },
            ProjectItem {
                name: "openresolve",
                link: "https://github.com/amaanbilwar/openresolve",
                prizes: vec![],
                description: vec![
                    "an open-source merge conflict resolution agent that proposes unified diffs instead of auto-applying edits",
                    "",
                    "integrated tree-sitter foundations with ast-level structural diff + overlap classification to support intent-aware candidate generation and safer merges",
                ],
                project_type: "personal",
                technologies: vec![
                    TypeScript::build(),
                    Bun::build(),
                ],
            },
            ProjectItem {
                name: "better-vscode",
                link: "https://github.com/amaanbilwar/better-vscode",
                prizes: vec![],
                description: vec![
                    "ship vscode customizations via patch files without vendoring upstream source",
                    "",
                    "automated patch workflows (sync/apply/reverse/dry-run) for clean rebases against microsoft/vscode",
                    "",
                    "disabled telemetry/experiments and removed ai chat ui for privacy and focus",
                ],
                project_type: "personal",
                technologies: vec![
                    TypeScript::build(),
                    Electron::build(),
                ],
            },
            ProjectItem {
                name: "zsh-zed",
                link: "https://github.com/amaanbilwar/zsh-zed",
                prizes: vec![],
                description: vec![
                    "zsh extension for the zed ide providing enhanced shell integration",
                    "",
                    "syntax highlighting and language support for zsh scripts in zed editor",
                ],
                project_type: "personal",
                technologies: vec![TypeScript::build()],
            },
            ProjectItem {
                name: "hql.nvim",
                link: "https://github.com/amaanbilwar/hql.nvim",
                prizes: vec![],
                description: vec![
                    "neovim plugin for hql (helix query language) syntax highlighting",
                    "",
                    "provides treesitter-based highlighting and lsp features for hql files",
                ],
                project_type: "personal",
                technologies: vec![TypeScript::build()],
            },
            ProjectItem {
                name: "helix-indexer",
                link: "https://github.com/amaanbilwar/helix-indexer",
                prizes: vec![],
                description: vec![
                    "codebase indexer for ai agents with semantic code understanding",
                    "",
                    "indexes codebases for efficient ai-powered search and navigation",
                ],
                project_type: "personal",
                technologies: vec![Rust::build(), TypeScript::build()],
            },
            ProjectItem {
                name: "vim-browser",
                link: "https://github.com/amaanbilwar/vim-extension",
                prizes: vec![],
                description: vec![
                    "vim keybindings for your browser - navigate the web with vim motions",
                    "",
                    "chrome extension providing vim-style navigation and shortcuts",
                ],
                project_type: "personal",
                technologies: vec![JavaScript::build()],
            },
        ];

        Self {
            state: 0,
            current_link: String::from(projects[0].link),
            projects,
            show_tooltip: false,
            tooltip_end_tick: 0,
            current_tick: 0,
        }
    }

    fn get_description(&self) -> Vec<Line<'_>> {
        let project_index = self.state;
        let mut final_vec: Vec<Line<'_>> = vec![];
        let project_item = &self.projects[project_index];

        for prize in &project_item.prizes {
            final_vec.push(line_from_spans(vec![gray_span(&prize)]));
        }

        if project_item.prizes.len() > 0 {
            final_vec.push(Line::from(""));
        }

        for desc_part in &project_item.description {
            final_vec.push(line_from_spans(vec![gray_span(&desc_part)]));
        }

        final_vec
    }

    fn previous_project(&mut self) {
        if self.state > 0 {
            self.state -= 1;
            self.change_current_link();
        }
    }

    fn next_project(&mut self) {
        if self.state < self.projects.len() - 1 {
            self.state += 1;
            self.change_current_link();
        }
    }

    fn change_current_link(&mut self) {
        self.current_link = String::from(self.projects[self.state].link);
    }
}
