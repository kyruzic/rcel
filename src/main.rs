mod events;
mod csv_utils;
mod widgets;

use std::io;
use std::iter;

use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::Constraint;
use tui::style::{Color, Style};
use tui::widgets::Widget;
use tui::{Frame, Terminal};

use events::{Event, Events};

use widgets::gridded_table::{GriddedTable, Row};


fn draw_table<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    // Getting data
    let csv_rows = csv_utils::read_csv_to_rows().unwrap();
    let headers = csv_utils::return_csv_headers();
    let num_columns = headers.len();
    let width = 100/ num_columns as u16;

    let rows = csv_rows.iter().map(|values| {
        Row::Data(values.iter())
    });

    let width_constraints = iter::repeat(width)
        .take(num_columns)
        .map(|w| Constraint::Percentage(w))
        .collect::<Vec<Constraint>>();

    // Rendering data
    let size = f.size();
    // TODO: This should be made using our own custom built widget that extends this table with the
    //       ability to draw borders and more
    GriddedTable::new(
            headers.iter(),
            rows.into_iter()
        )
        .widths(& width_constraints)
        .style(Style::default().fg(Color::White))
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
                draw_table(&mut f);
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

