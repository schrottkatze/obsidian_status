mod bar;
mod config;
mod formatting;

use crate::bar::bar::Bar;
use crate::bar::module::Module;
use formatting::ansi_format::AnsiFormat;

use formatting::colored::Colored;

fn main() {
    let bar = config::make_bar();

    bar.run()
}
