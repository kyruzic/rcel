pub mod gridded_table;

use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::Style;
use tui::symbols::line;
use tui::widgets::{Widget, Block};

use cassowary::strength::{MEDIUM, REQUIRED, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Expression, Solver};
