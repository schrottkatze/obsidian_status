// Imports {{{
extern crate battery;
extern crate terminal_size;
extern crate time;

use std::thread;
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};

mod bar;
mod config;
mod formatting;
mod module;
// }}}

fn main() {
    let statusbar = config::make_bar();

    loop {
        let size = terminal_size();

        if let Some((Width(w), Height(_h))) = size {
            statusbar.render(w);
        }

        thread::sleep(Duration::from_millis(config::UPDATE_MS));
    }
}
