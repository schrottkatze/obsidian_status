extern crate core;

mod bar;
mod config;
mod formatting;

use crate::bar::module::Module;
use crate::bar::Bar;
use formatting::ansi_format::AnsiFormat;

use formatting::colored::Colored;

fn main() {
    let bar = config::make_bar();

    bar.run()
}
