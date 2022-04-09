extern crate core;
extern crate network_interface;
extern crate sysinfo;

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
