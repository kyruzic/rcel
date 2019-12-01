use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::{Style, Color};
use tui::symbols::line;
use tui::widgets::{Block, Widget};

use cassowary::strength::{MEDIUM, REQUIRED, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Expression, Solver};

/// Holds data to be displayed in a Table widget
pub enum Row<D, I>
where
    D: Iterator<Item = I>,
    I: Display,
{
    Data(D),
    StyledData(D, Style),
}

/// A widget to display data in formatted columns
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Table, Row};
/// # use tui::layout::Constraint;
/// # use tui::style::{Style, Color};
/// # fn main() {
/// let row_style = Style::default().fg(Color::White);
/// Table::new(
///         ["Col1", "Col2", "Col3"].into_iter(),
///         vec![
///             Row::StyledData(["Row11", "Row12", "Row13"].into_iter(), row_style),
///             Row::StyledData(["Row21", "Row22", "Row23"].into_iter(), row_style),
///             Row::StyledData(["Row31", "Row32", "Row33"].into_iter(), row_style),
///             Row::Data(["Row41", "Row42", "Row43"].into_iter())
///         ].into_iter()
///     )
///     .block(Block::default().title("Table"))
///     .header_style(Style::default().fg(Color::Yellow))
///     .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
///     .style(Style::default().fg(Color::White))
///     .column_spacing(1);
/// # }
/// ```
pub struct GriddedTable<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Header row for all columns
    header: H,
    /// Style for the header
    header_style: Style,
    /// Width constraints for each column
    widths: &'a [Constraint],
    /// Space between each column
    column_spacing: u16,
    /// Data to display in each row
    rows: R,
}

impl<'a, T, H, I, D, R> Default for GriddedTable<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T> + Default,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>> + Default,
{
    fn default() -> GriddedTable<'a, T, H, I, D, R> {
        GriddedTable {
            block: None,
            style: Style::default(),
            header: H::default(),
            header_style: Style::default(),
            widths: &[],
            rows: R::default(),
            column_spacing: 1,
        }
    }
}

impl<'a, T, H, I, D, R> GriddedTable<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    pub fn new(header: H, rows: R) -> GriddedTable<'a, T, H, I, D, R> {
        GriddedTable {
            block: None,
            style: Style::default(),
            header,
            header_style: Style::default(),
            widths: &[],
            rows,
            column_spacing: 1,
        }
    }

    pub fn header<II>(mut self, header: II) -> GriddedTable<'a, T, H, I, D, R>
    where
        II: IntoIterator<Item = T, IntoIter = H>,
    {
        self.header = header.into_iter();
        self
    }

    pub fn header_style(mut self, style: Style) -> GriddedTable<'a, T, H, I, D, R> {
        self.header_style = style;
        self
    }

    pub fn widths(mut self, widths: &'a [Constraint]) -> GriddedTable<'a, T, H, I, D, R> {
        let between_0_and_100 = |&w| match w {
            Constraint::Percentage(p) => p <= 100,
            _ => true,
        };
        assert!(
            widths.iter().all(between_0_and_100),
            "Percentages should be between 0 and 100 inclusively."
        );
        self.widths = widths;
        self
    }

    pub fn rows<II>(mut self, rows: II) -> GriddedTable<'a, T, H, I, D, R>
    where
        II: IntoIterator<Item = Row<D, I>, IntoIter = R>,
    {
        self.rows = rows.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> GriddedTable<'a, T, H, I, D, R> {
        self.style = style;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> GriddedTable<'a, T, H, I, D, R> {
        self.column_spacing = spacing;
        self
    }
}

impl<'a, T, H, I, D, R> Widget for GriddedTable<'a, T, H, I, D, R>
where
    T: Display,
    H: Iterator<Item = T>,
    I: Display,
    D: Iterator<Item = I>,
    R: Iterator<Item = Row<D, I>>,
{
    fn draw(&mut self, table_area: Rect, buf: &mut Buffer) {
        // Render block if necessary and get the drawing area


        // Set the background
        self.background(table_area, buf, self.style.bg);

        let mut solver = Solver::new();
        let mut var_indices = HashMap::new();
        let mut ccs = Vec::new();
        let mut variables = Vec::new();
        for i in 0..self.widths.len() {
            let var = cassowary::Variable::new();
            variables.push(var);
            var_indices.insert(var, i);
        }
        for (i, constraint) in self.widths.iter().enumerate() {
            ccs.push(variables[i] | GE(WEAK) | 0.);
            ccs.push(match *constraint {
                Constraint::Length(v) => variables[i] | EQ(MEDIUM) | f64::from(v),
                Constraint::Percentage(v) => {
                    variables[i] | EQ(WEAK) | (f64::from(v * table_area.width) / 100.0)
                }
                Constraint::Ratio(n, d) => {
                    variables[i] | EQ(WEAK) | (f64::from(table_area.width) * f64::from(n) / f64::from(d))
                }
                Constraint::Min(v) => variables[i] | GE(WEAK) | f64::from(v),
                Constraint::Max(v) => variables[i] | LE(WEAK) | f64::from(v),
            })
        }
        solver
            .add_constraint(
                variables
                    .iter()
                    .fold(Expression::from_constant(0.), |acc, v| acc + *v)
                    | LE(REQUIRED)
                    | f64::from(
                        table_area.width - 2 - (self.column_spacing * (variables.len() as u16 - 1)),
                    ),
            )
            .unwrap();
        solver.add_constraints(&ccs).unwrap();
        // TODO: Figure out why if there is a large number of columns solver.fetch_changes()
        // returns a different value for the last column than all of the other columns
        let mut solved_widths = vec![0; variables.len()];
        for &(var, value) in solver.fetch_changes() {
            let index = var_indices[&var];
            let value = if value.is_sign_negative() {
                0
            } else {
                value as u16
            };
            solved_widths[index] = value
        }

        // let mut x = 0;
        // let mut widths = Vec::with_capacity(self.widths.len());
        // for width_percentage in self.widths.iter() {
        //     let width = ((*width_percentage as f32 * 0.01) * table_area.width as f32) as u16;
        //     if x + width < table_area.width {
        //         widths.push(width);
        //     }
        //     x += width;
        // }


        let mut y = table_area.top();
        let mut x = table_area.left();

        // Draw header
        if y < table_area.bottom() {
            for (w, t) in solved_widths.iter().zip(self.header.by_ref()) {
                buf.set_stringn(
                    x + self.column_spacing + 1,
                    y + 1,
                    format!("{}", t),
                    *w as usize,
                    self.header_style,
                );
                x += *w + self.column_spacing;
            }
        }
        // Draw all three horiztonal lines, An above header line, a below header line, and a bottom of the table line
        for x1 in table_area.left()..table_area.right() {
            buf.get_mut(x1, table_area.top())
                .set_symbol(line::HORIZONTAL);
            buf.get_mut(x1, table_area.top() + 2)
                .set_symbol(line::HORIZONTAL);
            buf.get_mut(x1, table_area.bottom() - 1)
                .set_symbol(line::HORIZONTAL);
        }

        // Increment y to be beneath an horizontal line
        // Empty row + horizontal line + empty row = 3

        for y1 in table_area.top()..table_area.bottom() {
            if y1 == table_area.top() + 2 {
                buf.get_mut(table_area.right() - 1, y1)
                    .set_symbol(line::VERTICAL_LEFT);
                buf.get_mut(table_area.left(), y1)
                    .set_symbol(line::VERTICAL_RIGHT);
            }
            else {
                buf.get_mut(table_area.right() - 1, y1)
                    .set_symbol(line::VERTICAL);
                buf.get_mut(table_area.left(), y1)
                    .set_symbol(line::VERTICAL);
            }
        }

        y += 3;

        // Draw rows
        let default_style = Style::default();
        if y < table_area.bottom() {
            let remaining = (table_area.bottom() - y - 1) as usize;

            for (i, row) in self.rows.by_ref().take(remaining).enumerate() {
                let (data, style) = match row {
                    Row::Data(d) => (d, default_style),
                    Row::StyledData(d, s) => (d, s),
                };
                x = table_area.left();

                for (w, elt) in solved_widths.iter().zip(data) {
                    // Draw the verticle line between columns
                    if x > 0 {
                        for y in table_area.top()..table_area.bottom(){
                            if y == table_area.top() {
                                buf.get_mut(table_area.left() + x, y)
                                    .set_symbol(line::HORIZONTAL_DOWN);
                            }
                            else if y == table_area.bottom() - 1 {
                                buf.get_mut(table_area.left() + x, y)
                                    .set_symbol(line::HORIZONTAL_UP);
                            }
                            else if y == table_area.top() + 2 {
                                buf.get_mut(table_area.left() + x, y)
                                    .set_symbol(line::CROSS);
                            }
                            else {
                                buf.get_mut(table_area.left() + x, y)
                                    .set_symbol(line::VERTICAL);
                            }
                        }
                    }

                    // We begin writing text two characters after the column begins
                    // This leaves space for the vertical line separator and an empty
                    // space in front of it
                    buf.set_stringn(
                        x + 2,
                        y + i as u16,
                        format!("{}", elt),
                        *w as usize,
                        style,
                    );
                    // Move the x position to the beginning of the next column
                    x += *w + self.column_spacing;
                }
            }
            // Draw Table corners
            buf.get_mut(table_area.left(), table_area.top())
                .set_symbol(line::TOP_LEFT);
            buf.get_mut(table_area.right() - 1, table_area.top())
                .set_symbol(line::TOP_RIGHT);
            buf.get_mut(table_area.left(), table_area.bottom() - 1)
                .set_symbol(line::BOTTOM_LEFT);
            buf.get_mut(table_area.right() - 1, table_area.bottom() - 1)
                .set_symbol(line::BOTTOM_RIGHT);
        }
    }
}
