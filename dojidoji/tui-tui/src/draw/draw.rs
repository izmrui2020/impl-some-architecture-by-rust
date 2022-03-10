//
use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use std::string::ToString;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::{self, Backend, TermionBackend};
use tui::widgets::{Block, List, Tabs, Paragraph, Borders, BorderType};
use tui::text::{Span, Spans, Text};
use tui::style::{Style, Color, Modifier};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use strum::IntoEnumIterator;
use super::MenuItem;
use super::HomeItem;
use super::sub::SubCenter;

use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use std::io::{self, Write, Stdout};

type Termon = TermionBackend<RawTerminal<Stdout>>;

pub struct Termondayo{
    pub terminal: Terminal<Termon>,
    pub home: Vec<&'static str>,
    pub select: HomeItem,
}

impl Termondayo {
    pub fn new() -> Self {
        let stdout = io::stdout().into_raw_mode().expect("failed at stdout");
        // let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);

        Self {
            terminal: Terminal::new(backend).expect("failed terminal new"),
            home: vec!["Home", "Ranking", "Select"],
            select: HomeItem::Home,
        }
    }

    pub async fn draw(&mut self) -> Result<()> {

        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                //.margin(1)
                .constraints(
                    [
                        Constraint::Percentage(1),
                        Constraint::Percentage(14),
                        Constraint::Percentage(1),
                        Constraint::Percentage(84),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let menu = self.home.iter()
                .map(|i| {
                    let (first, rest) = i.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();
            
            let tabs = Tabs::new(menu)
                .select(self.select.into())
                .block(
                    Block::default()
                    .title("Termondayo")
                    .borders(Borders::ALL))
                .style(Style::default().fg(Color::LightGreen))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));
            
                f.render_widget(tabs, chunks[1]);

            match self.select {
                HomeItem::Home => f.render_widget(home_widget(), chunks[3]),
                HomeItem::Ranking => {
                    let ranking_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref(),
                        ).split(chunks[3]);
                    
                    },
                HomeItem::Select => todo!(),
            }
        })?;
        Ok(())
    }
}

pub fn home_widget<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightGreen))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}