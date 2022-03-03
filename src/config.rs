use std::process::Command;
use std::time::SystemTime;

use time::{format_description, OffsetDateTime};

use termcolor::{Color, ColorSpec};

use super::bar::{Bar, BarConfig, SepSet};
use super::module::{JustifyModule, Module};

pub const UPDATE_MS: u64 = 1000;

pub fn make_bar() -> Bar {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Rgb(127, 0, 255)));

    let mut sep_color_spec = ColorSpec::new();
    sep_color_spec.set_fg(Some(Color::Rgb(0, 127, 255)));

    let mut bar = Bar::new(BarConfig {
        sep_set: SepSet::SingleSidesDualCenter(
            ">".to_string(),
            ("(".to_string(), ")".to_string()),
            "<".to_string(),
        ),
        color_spec,
        sep_color_spec,
        show_both_seps_on_overlap: true,
        outer_sep_config: (true, true),
    });

    // clock example
    bar.add_module(
        JustifyModule::Center,
        Module::new(20, clock_mod, None, None, None),
    );

    // ping example
    bar.add_module(
        JustifyModule::Left,
        Module::new(27, ping_cf_mod, None, None, None),
    );

    // testing
    bar.add_module(
        JustifyModule::Right,
        Module::new(27, ping_cf_mod, None, None, None),
    );
    bar.add_module(
        JustifyModule::Right,
        Module::new(20, clock_mod, None, None, None),
    );

    bar
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
