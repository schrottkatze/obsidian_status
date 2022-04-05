mod bar;
mod config;
mod formatting;

use crate::bar::bar::Bar;
use crate::bar::module::Module;
use formatting::ansi_format::AnsiFormat;
use formatting::color::Color;
use formatting::colored::{Colored, ColoredString};
use std::rc::Rc;
use std::time::Duration;

fn main() {
    let mut bar = config::make_bar();
}
