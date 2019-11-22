use std::error::Error;
use std::process;
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::Frame;
use tui::backend::Backend;


fn return_csv_headers() -> Vec<String> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("/home/kyle/datafiles/csvTest/uspop.csv").unwrap();
    let headers = rdr.headers().unwrap();
    headers.iter().map(|x| x.to_string()).collect()
}

fn draw_headers<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let headers = return_csv_headers();
    let width = 100/headers.len() as u16;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(width),
                Constraint::Percentage(width),
                Constraint::Percentage(width),
                Constraint::Percentage(width),
                Constraint::Percentage(width)
            ].as_ref()
        )
        .split(f.size());
    Block::default()
        .title(&headers[0])
        .borders(Borders::ALL)
        .render(f, chunks[0]);
    Block::default()
        .title(&headers[1])
        .borders(Borders::ALL)
        .render(f, chunks[1]);
    Block::default()
        .title(&headers[2])
        .borders(Borders::ALL)
        .render(f, chunks[2]);
    Block::default()
        .title(&headers[3])
        .borders(Borders::ALL)
        .render(f, chunks[3]);
    Block::default()
        .title(&headers[4])
        .borders(Borders::ALL)
        .render(f, chunks[4]);
}

fn main() -> Result<(), io::Error>{
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|mut f| {
        draw_headers(&mut f);
    })
}
