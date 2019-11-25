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
use tui::widgets::{Block, Borders, Widget};
use tui::{Frame, Terminal};

use events::{Event, Events};

fn draw_rows<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let columns = csv_utils::read_csv_to_columns().unwrap();
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
        .title(&columns.get("City").unwrap()[0])
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[0]);
    Block::default()
        .title(&columns.get("State").unwrap()[0])
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[1]);
    Block::default()
        .title(&columns.get("Population").unwrap()[0])
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[2]);
    Block::default()
        .title(&columns.get("Latitude").unwrap()[0])
        .borders(Borders::LEFT | Borders::BOTTOM)
        .render(f, chunks[3]);
    Block::default()
        .title(&columns.get("Longitude").unwrap()[0])
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

// fn main() {
//     let mut rdr = csv::Reader::from_path("./assets/uspop.csv").unwrap();
//     for result in rdr.deserialize() {
//         let record: HashMap<String, String> = result.unwrap();
//         println!("{:?}", record)
//     }
//     let columns = csv_utils::read_csv_to_columns();
//     for column in columns  {
//         println!("{:?}", column)
//     }
// }
