//
//use anyhow::Result;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::{self, Backend, TermionBackend};
use tui::widgets::{Block, List, Tabs, Paragraph, Borders, BorderType};
use tui::text::{Span, Spans, Text};
use tui::style::{Style, Color, Modifier};
use tui::layout::{Layout, Direction, Constraint, Alignment};

use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use std::io::{self, Write, Stdout};

type Termon = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;

pub struct Termondayo{
    pub terminal: Terminal<Termon>,
}

impl Termondayo {
    pub fn new() -> Self {
        let stdout = io::stdout().into_raw_mode().expect("failed at stdout");
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        
        Self {
            terminal: Terminal::new(backend).expect("failed terminal new")
        }
    }

    pub fn draw(&mut self) {

        self.terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                //.margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Percentage(2),
                        Constraint::Percentage(2),
                    ]
                    .as_ref(),
                )
                .split(size);

            let title = Paragraph::new("This is Termondayo")
                .style(Style::default().fg(Color::LightGreen))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
                );

            f.render_widget(title, chunks[1]);

            let header = ["Header1", "Header2", "Header3"];
        });
    }
}