mod ansi_format;
mod colored;

use crate::ansi_format::{AnsiFormat, NoResetFormatConf};
use ansi_format::Color;

fn main() {
    println!(
        "{}hii{}byee",
        AnsiFormat::with_fg(Color::Blue).get_code(),
        AnsiFormat::with_bg(Color::Red).get_code()
    )
}
