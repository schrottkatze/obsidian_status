use std::process::Command;
use std::time::SystemTime;

use time::{format_description, OffsetDateTime};

pub const UPDATE_MS: u64 = 1000;

pub fn make_bar() -> Bar {
}

fn clock_mod() -> String {
    let tfmt = format_description::parse("[year]-[month]-[day], [hour]:[minute]:[second]").unwrap();
    let systime: OffsetDateTime = SystemTime::now().into();

    systime.format(&tfmt).unwrap()
}

fn ping_cf_mod() -> String {
    let ip = "1.1.1.1";

    let ping_output = Command::new("/usr/bin/ping")
        .args(["-c", "1", ip])
        .output()
        .expect("ping failed!")
        .stdout;

    let r = std::str::from_utf8(&ping_output)
        .unwrap()
        .to_string()
        .lines()
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string();

    let ms_pos = r.find("time=").unwrap() + 5;
    let time = &r[ms_pos..r.len()];

    format!(
        "Ping to {}:{}{}",
        ip,
        " ".repeat(4 - (time.len() - 7)),
        time
    )
}
