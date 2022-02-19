use std::thread;
use std::time;

extern crate terminal_size;
use terminal_size::{terminal_size, Height, Width};

mod config;


fn main() {
    loop {
        let size = terminal_size();

        if let Some((Width(_w), Height(_h))) = size {
        }

        thread::sleep(time::Duration::from_millis(config::UPDATE_MS));
    println!("Hello, world!");
    }

}

