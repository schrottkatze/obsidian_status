
// Imports {{{
use std::time::{Duration, SystemTime};

use time::{format_description, OffsetDateTime};

use crate::{Bar, Colored, Module};
use battery::{Battery, State};
// }}}

pub const UPDATE_MS: u64 = 1000;

pub fn make_bar() -> Bar {
    let mut r = Bar::new(Duration::from_millis(UPDATE_MS));

    r.push_module(Module::DynSpacer);
    r.push_module(Module::new_single_threaded(clock_mod));
    r.push_module(Module::DynSpacer);
    r.push_module(Module::new_multi_threaded(battery_mod, true));

    r
}

fn clock_mod() -> Colored {
    let time_fmt =
        format_description::parse("[year]-[month]-[day], [hour]:[minute]:[second]").unwrap();
    let systime: OffsetDateTime = SystemTime::now().into();

    Colored::from_str(&systime.format(&time_fmt).unwrap())
}

fn battery_mod(_prev: Option<Vec<Colored>>) -> Colored {
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

            Colored::from_str(&format!(
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
            ))
        }
        None => Colored::from_str("No battery found!"),
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
