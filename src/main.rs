mod events;

use std::error::Error;
use std::io;
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::{Frame, Terminal};

use events::{Event, Events};

fn return_csv_headers() -> Vec<String> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("./assets/uspop.csv").unwrap();
    let headers = rdr.headers().unwrap();
    headers.iter().map(|x| x.to_string()).collect()
}

fn draw_headers<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let headers = return_csv_headers();
    let width = 100 / headers.len() as u16;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(width),
                Constraint::Percentage(width),
                Constraint::Percentage(width),
                Constraint::Percentage(width),
                Constraint::Percentage(width),
            ]
            .as_ref(),
        )
        .split(f.size());
    Block::default()
        .title(&headers[0])
        .title_style(Style::default().fg(Color::Red))
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .render(f, chunks[0]);
    Block::default()
        .title(&headers[1])
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .render(f, chunks[1]);
    Block::default()
        .title(&headers[2])
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .render(f, chunks[2]);
    Block::default()
        .title(&headers[3])
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .render(f, chunks[3]);
    Block::default()
        .title(&headers[4])
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .render(f, chunks[4]);
}

fn main() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    let events = Events::new();

    loop {
        terminal
            .draw(|mut f| {
                draw_headers(&mut f);
            })
            .unwrap();

        match events.next().unwrap() {
            Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }
            }
            Event::Tick => {}
        }
    }
}
