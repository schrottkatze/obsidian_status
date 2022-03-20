use std::time::SystemTime;

use time::{format_description, OffsetDateTime};

use super::bar::{Bar, SegSepTypes, Segment};
use super::formatting::colored::Colored;
use super::formatting::text_format_conf::{Color, TextFormatConf};
use super::module::Module;

use battery::{Battery, State};

pub const UPDATE_MS: u64 = 1000;

pub fn make_bar() -> Bar {
    let mut r = Bar::new((true, false));

    r.add_segment(Segment::DynSpacer)
        .add_segment(Segment::StatusSeg(
            vec![Module::new(
                27,
                clock_mod,
                Some(TextFormatConf::fg_and_bg(
                    Color::Rgb((0x28, 0x28, 0x28)),
                    Color::Rgb((0xb1, 0x62, 0x86)),
                )),
                None,
            )],
            SegSepTypes::Two(
                Colored::new(
                    "",
                    TextFormatConf::fg_and_bg(
                        Color::Rgb((0xb1, 0x62, 0x86)),
                        Color::Rgb((0x28, 0x28, 0x28)),
                    ),
                    false,
                ),
                Colored::new(
                    "",
                    TextFormatConf::fg_and_bg(
                        Color::Rgb((0xb1, 0x62, 0x86)),
                        Color::Rgb((0x28, 0x28, 0x28)),
                    ),
                    true,
                ),
            ),
        ))
        .add_segment(Segment::DynSpacer)
        .add_segment(Segment::StatusSeg(
            vec![Module::new(
                150,
                battery_mod,
                Some(TextFormatConf::fg_and_bg(
                    Color::Rgb((0x28, 0x28, 0x28)),
                    Color::Rgb((0xb1, 0x62, 0x86)),
                )),
                None,
            )],
            SegSepTypes::Three(
                Colored::new(
                    "",
                    TextFormatConf::fg_and_bg(
                        Color::Rgb((0xb1, 0x62, 0x86)),
                        Color::Rgb((0x28, 0x28, 0x28)),
                    ),
                    false,
                ),
                Colored::new(
                    "",
                    TextFormatConf::fg_and_bg(
                        Color::Rgb((0x28, 0x28, 0x28)),
                        Color::Rgb((0xb1, 0x62, 0x86)),
                    ),
                    false,
                ),
                Colored::new(
                    "",
                    TextFormatConf::fg_and_bg(
                        Color::Rgb((0xb1, 0x62, 0x86)),
                        Color::Rgb((0x28, 0x28, 0x28)),
                    ),
                    true,
                ),
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

fn battery_mod() -> String {
    let manager = battery::Manager::new().unwrap();
    let mut bats: Vec<(usize, Battery)> = vec![];

    for (idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
        let battery = maybe_battery.unwrap();
        bats.push((idx, battery))
    }

    match bats.get(0) {
        Some(bat) => {
            let bat_state: f32 = bat.1.state_of_charge().into();
            let icon = String::from(match bat.1.state() {
                State::Charging => {
                    if bat_state < 0.2 {
                        ''
                    } else if bat_state < 0.3 {
                        ''
                    } else if bat_state < 0.4 {
                        ''
                    } else if bat_state < 0.6 {
                        ''
                    } else if bat_state < 0.8 {
                        ''
                    } else if bat_state < 0.9 {
                        ''
                    } else if bat_state < 1.0 {
                        ''
                    } else {
                        '?'
                    }
                }
                State::Discharging | State::Full => {
                    if bat_state < 0.1 {
                        ''
                    } else if bat_state < 0.2 {
                        ''
                    } else if bat_state < 0.3 {
                        ''
                    } else if bat_state < 0.4 {
                        ''
                    } else if bat_state < 0.5 {
                        ''
                    } else if bat_state < 0.6 {
                        ''
                    } else if bat_state < 0.7 {
                        ''
                    } else if bat_state < 0.8 {
                        ''
                    } else if bat_state < 0.9 {
                        ''
                    } else if bat_state < 1.0 {
                        ''
                    } else {
                        '?'
                    }
                }
                State::Empty => '',
                _ => '',
            });

            format!(
                "{} {}%{}",
                icon,
                (bat_state * 100.0).floor(),
                match bat.1.state() {
                    State::Discharging =>
                        if let Some(t) = bat.1.time_to_empty() {
                            let t_int = t.value as u64;
                            format_to_h_m(t_int)
                        } else {
                            "".to_string()
                        },
                    State::Charging =>
                        if let Some(t) = bat.1.time_to_full() {
                            let t_int = t.value as u64;
                            format_to_h_m(t_int)
                        } else {
                            "".to_string()
                        },
                    _ => "".to_string(),
                }
            )
        }
        None => String::from("No battery!"),
    }
}

fn format_to_h_m(t: u64) -> String {
    let h = t / 3600;
    let m = (t % 3600) / 60;

    format!(
        " {}:{}",
        if h > 10 {
            h.to_string()
        } else {
            format!("0{}", h)
        },
        if m > 10 {
            m.to_string()
        } else {
            format!("0{}", m)
        }
    )
}

