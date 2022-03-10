use std::process::Command;
use std::time::SystemTime;

use time::{format_description, OffsetDateTime};

use super::bar::{Bar, SegSepTypes, Segment};
use super::formatting::colored::Colored;
use super::formatting::text_format_conf::{Color, TextFormatConf};
use super::module::Module;

pub const UPDATE_MS: u64 = 1000;

pub fn make_bar() -> Bar {
    let mut r = Bar::new((true, false));

    r.add_segment(Segment::DynSpacer).add_segment(Segment::StatusSeg(
        vec![
            Module::new(27, clock_mod, None),
        ],
        SegSepTypes::Two(
            Colored::new("<", TextFormatConf::fg_only(Color::HCRed)),
            Colored::new(">", TextFormatConf::fg_only(Color::HCGreen)),
        ),
    ))
    .add_segment(Segment::DynSpacer)
    .add_segment(Segment::StatusSeg(
        vec![
            Module::new(27, ping_cf_mod, None),
        ],
        SegSepTypes::Two(
            Colored::new("<", TextFormatConf::fg_only(Color::HCRed)),
            Colored::new(">", TextFormatConf::fg_only(Color::HCGreen)),
        ),
    ));

    r
}

// len is 20
fn clock_mod() -> String {
    let tfmt = format_description::parse("[year]-[month]-[day], [hour]:[minute]:[second]").unwrap();
    let systime: OffsetDateTime = SystemTime::now().into();

    systime.format(&tfmt).unwrap()
}

// len is 26
// TODO: Fix its hilarious instability
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
        "{}:{}{}",
        ip,
        " ".repeat(8 - (time.len() - 3)),
        time
    )
}
