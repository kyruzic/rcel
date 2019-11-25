mod events;
mod csv_utils;

use std::error::Error;
use std::io;
use std::process;
use std::iter;
use std::collections::HashMap;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget, Table, Row};
use tui::{Frame, Terminal};

use events::{Event, Events};

fn draw_rows<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    // Getting data
    let columns = csv_utils::read_csv_to_columns().unwrap();
    let csv_rows = csv_utils::read_csv_to_rows().unwrap();
    let headers = csv_utils::return_csv_headers();

    let row_style = Style::default().fg(Color::Red);
   
    let rows = csv_rows.iter().map(|values| {
        Row::StyledData(values.iter(), row_style)
    });

    // Rendering data
    let width = 20;
    let constraints_vec = iter::repeat(width)
        .take(columns.keys().len())
        .map(|w| Constraint::Percentage(w))
        .collect::<Vec<Constraint>>();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            constraints_vec.as_ref(),
        )
        .split(f.size());

    Block::default()
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[0]);
    Block::default()
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[1]);
    Block::default()
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[2]);
    Block::default()
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[3]);
    Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .render(f, chunks[4]);

    let size = f.size();
    Table::new(
            headers.iter(),
            rows.into_iter()
        )
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[20, 20, 20, 20, 20])
        .style(Style::default().fg(Color::White))
        .column_spacing(2)
        .render(f, size);

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
                draw_rows(&mut f);
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

