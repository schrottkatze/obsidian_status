use std::collections::VecDeque;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::{slice, thread};

use time::{format_description, OffsetDateTime};

use crate::bar::module::background::{Background, BackgroundRunFn};
use crate::formatting::color::Color;
use crate::formatting::colored::ColoredString;
use crate::{AnsiFormat, Bar, Colored, Module};
use battery::{Battery, State};
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};
use sysinfo::{Disk, DiskExt, ProcessorExt, System, SystemExt};

pub const UPDATE_MS: u64 = 1000;
const SHOW_IPV6: bool = false;

pub fn make_bar() -> Bar {
    let mut r = Bar::new(Duration::from_millis(UPDATE_MS));

    r.push_module(Module::new_background(sys_cpu));
    r.push_module(Module::new_background(sys_mem));
    r.push_module(Module::DynSpacer);
    r.push_module(Module::new_single_threaded(clock_mod));
    r.push_module(Module::DynSpacer);
    r.push_module(Module::new_background(w_ip_mod));
    r.push_module(Module::new_background(battery_mod));

    r
}

fn clock_mod() -> Colored {
    let time_fmt =
        format_description::parse("[year]-[month]-[day], [hour]:[minute]:[second]").unwrap();
    let systime: OffsetDateTime = SystemTime::now().into();

    let mut r = Colored::new();

    r.push_el(ColoredString::new(
        "",
        None,
        Some(AnsiFormat::fg_and_bg(
            Color::Rgb((0x28, 0x28, 0x28)),
            Color::Rgb((0xb8, 0xbb, 0x26)),
        )),
    ));
    r.push_el(ColoredString::new(
        &systime.format(&time_fmt).unwrap(),
        None,
        None,
    ));
    r.push_el(ColoredString::new(
        "",
        Some(AnsiFormat::fg_and_bg(
            Color::Rgb((0xb8, 0xbb, 0x26)),
            Color::Rgb((0x68, 0x9d, 0x6a)),
        )),
        None,
    ));

    r
}

fn battery_mod(current: Arc<Mutex<Option<Colored>>>, tx: Sender<()>) {
    let manager = battery::Manager::new().unwrap();
    let mut bats: Vec<(usize, Battery)> = vec![];

    loop {
        for (idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
            let battery = maybe_battery.unwrap();
            bats.push((idx, battery))
        }

        let r = match bats.get(0) {
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

                let mut r = Colored::new();

                r.push_el(ColoredString::new(
                    "",
                    Some(AnsiFormat::fg_and_bg(
                        Color::Rgb((0x98, 0x97, 0x1a)),
                        Color::Rgb((0xb8, 0xbb, 0x26)),
                    )),
                    Some(AnsiFormat::fg_and_bg(
                        Color::Rgb((0x28, 0x28, 0x28)),
                        Color::Rgb((0x98, 0x97, 0x1a)),
                    )),
                ));
                r.push_el(ColoredString::new(
                    &format!(
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
                    ),
                    None,
                    None,
                ));

                r
            }
            None => Colored::from_str("No battery found!"),
        };

        let mut current = current.lock().unwrap();
        *current = Some(r);
        drop(current);

        thread::sleep(Duration::from_millis(10000))
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

fn w_ip_mod(current: Arc<Mutex<Option<Colored>>>, tx: Sender<()>) {
    let mut prev_addrs = None;

    fn get_wlan0_addrs(interfaces: Vec<NetworkInterface>) -> Vec<Addr> {
        let mut r = Vec::<Addr>::new();

        for itf in interfaces {
            if itf.name == "wlan0".to_string() {
                if let Some(addr) = itf.addr {
                    r.push(addr)
                }
            }
        }

        r
    }

    fn check_for_change(prev_addrs: &Option<Vec<Addr>>) -> Option<Vec<Addr>> {
        let new_addrs = get_wlan0_addrs(NetworkInterface::show().unwrap());

        if let Some(prev_addrs) = prev_addrs {
            if new_addrs.len() != prev_addrs.len() {
                Some(new_addrs)
            } else {
                let mut equal = true;

                for (i, addr) in new_addrs.iter().enumerate() {
                    let prev_addr_i = &prev_addrs[i];

                    equal = match addr {
                        Addr::V4(new_addr) => match prev_addr_i {
                            Addr::V4(prev_addr) => new_addr.ip == prev_addr.ip,
                            Addr::V6(_) => false,
                        },
                        Addr::V6(new_addr) => match prev_addr_i {
                            Addr::V6(prev_addr) => new_addr.ip == prev_addr.ip,
                            Addr::V4(_) => false,
                        },
                    };
                }

                if !equal {
                    Some(new_addrs)
                } else {
                    None
                }
            }
        } else {
            Some(new_addrs)
        }
    }

    loop {
        if let Some(addr) = check_for_change(&prev_addrs) {
            let mut r = Colored::new();

            r.push_el(ColoredString::new(
                "",
                None,
                Some(AnsiFormat::fg_and_bg(
                    Color::Rgb((0x28, 0x28, 0x28)),
                    Color::Rgb((0xb8, 0xbb, 0x26)),
                )),
            ));
            r.push_el(ColoredString::new("直 ", None, None));

            if addr.is_empty() {
                r.push_el(ColoredString::new(
                    "Down",
                    Some(AnsiFormat::with_fg(Color::Red)),
                    None,
                ));
            } else {
                for addr in &addr {
                    match addr {
                        Addr::V4(ip) => {
                            r.push_el(ColoredString::new(&ip.ip.to_string(), None, None))
                        }
                        Addr::V6(ip) => {
                            if SHOW_IPV6 {
                                r.push_el(ColoredString::new(&ip.ip.to_string(), None, None))
                            }
                        }
                    }
                }
            }
            r.push_el(ColoredString::new(" ", None, None));

            let mut current_locked = current.lock().unwrap();
            *current_locked = Some(r);
            tx.send(());
            prev_addrs = Some(addr);
        }
        thread::sleep(Duration::from_millis(5000))
    }
}

const GIB_DIVIDER: f64 = 1024.0 * 1024.0;
// const GIG_DIVIDER: f64 = 1000.0 * 1000.0;
fn sys_cpu(current: Arc<Mutex<Option<Colored>>>, tx: Sender<()>) {
    let mut sys = System::new();
    let mut deque = VecDeque::<f64>::with_capacity(16);
    while deque.len() < 16 {
        deque.push_back(0.0)
    }

    loop {
        sys.refresh_system();
        let mut r = Colored::new();

        let processors = sys.get_processor_list();
        let mut usage = 0.0;
        for cpu in processors {
            usage += cpu.get_cpu_usage();
        }
        let usage = usage / 8.0;
        deque.pop_back();
        deque.push_front(usage as f64);

        r.push_el(ColoredString::new(
            " [",
            Some(AnsiFormat::fg_and_bg(
                Color::Rgb((0x28, 0x28, 0x28)),
                Color::Rgb((0x98, 0x97, 0x1a)),
            )),
            None,
        ));
        r.push_colored(&mut braille_graph(&deque, 8));
        r.push_el(ColoredString::new("] ", None, None));

        r.push_el(ColoredString::new(
            &format!(
                "{}%, {} CPUs",
                round((usage as f64) * 100.0, 1),
                processors.len()
            ),
            None,
            None,
        ));
        r.push_el(ColoredString::new(
            "",
            Some(AnsiFormat::fg_and_bg(
                Color::Rgb((0x98, 0x97, 0x1a)),
                Color::Rgb((0xb8, 0xbb, 0x26)),
            )),
            Some(AnsiFormat::fg_and_bg(
                Color::Rgb((0x28, 0x28, 0x28)),
                Color::Rgb((0xb8, 0xbb, 0x26)),
            )),
        ));

        let mut current = current.lock().unwrap();
        *current = Some(r);
        drop(current);
        thread::sleep(Duration::from_millis(1000));
    }
}
fn sys_mem(current: Arc<Mutex<Option<Colored>>>, tx: Sender<()>) {
    let mut sys = System::new();
    let mem_total = sys.get_total_memory();
    let mut deque = VecDeque::<f64>::with_capacity(16);
    while deque.len() < 16 {
        deque.push_back(0.0)
    }

    loop {
        sys.refresh_system();
        let mem_used = sys.get_used_memory();

        let mut r = Colored::new();
        let frac = mem_used as f64 / mem_total as f64;

        deque.pop_back();
        deque.push_front(frac);

        r.push_el(ColoredString::new(" [", None, None));
        r.push_colored(&mut braille_graph(&deque, 8));
        r.push_el(ColoredString::new("] ", None, None));

        r.push_el(ColoredString::new(
            &format!(
                "{}% ({}/{} GiB)",
                round(frac * 100.0, 1),
                round(mem_used as f64 / GIB_DIVIDER, 1),
                round(mem_total as f64 / GIB_DIVIDER, 1)
            ),
            None,
            None,
        ));
        r.push_el(ColoredString::new(
            "",
            Some(AnsiFormat::Reset),
            Some(AnsiFormat::fg_and_bg(
                Color::Rgb((0xb8, 0xbb, 0x26)),
                Color::Rgb((0x68, 0x9d, 0x6a)),
            )),
        ));
        r.push_el(ColoredString::new("", None, None));

        let mut current = current.lock().unwrap();
        *current = Some(r);
        drop(current);

        thread::sleep(Duration::from_millis(1000));
    }
}

fn braille_graph(vals: &VecDeque<f64>, width: u16) -> Colored {
    let mut r = Colored::new();
    let len = (vals.len() - 1) as u16;

    for i in 0..width {
        let vals = (
            vals[(len - (i * 2)) as usize],
            vals[(len - (i * 2) - 1) as usize],
        );

        r.push_el(ColoredString::new(
            if vals.0 < 0.2 {
                if vals.1 < 0.2 {
                    "⠀"
                } else if vals.1 < 0.4 {
                    "⢀"
                } else if vals.1 < 0.6 {
                    "⢠"
                } else if vals.1 < 0.8 {
                    "⢰"
                } else if vals.1 < 1.0 {
                    "⢸"
                } else {
                    panic!(
                        "Value at position {} is out of range: {}",
                        len - (i * 2) + 1,
                        vals.1
                    )
                }
            } else if vals.0 < 0.4 {
                if vals.1 < 0.2 {
                    "⡀"
                } else if vals.1 < 0.4 {
                    "⣀"
                } else if vals.1 < 0.6 {
                    "⣠"
                } else if vals.1 < 0.8 {
                    "⣰"
                } else if vals.1 < 1.0 {
                    "⣸"
                } else {
                    panic!(
                        "Value at position {} is out of range: {}",
                        len - (i * 2) + 1,
                        vals.1
                    )
                }
            } else if vals.0 < 0.6 {
                if vals.1 < 0.2 {
                    "⡄"
                } else if vals.1 < 0.4 {
                    "⣄"
                } else if vals.1 < 0.6 {
                    "⣤"
                } else if vals.1 < 0.8 {
                    "⣴"
                } else if vals.1 < 1.0 {
                    "⣼"
                } else {
                    panic!(
                        "Value at position {} is out of range: {}",
                        len - (i * 2) + 1,
                        vals.1
                    )
                }
            } else if vals.0 < 0.8 {
                if vals.1 < 0.2 {
                    "⡆"
                } else if vals.1 < 0.4 {
                    "⣆"
                } else if vals.1 < 0.6 {
                    "⣦"
                } else if vals.1 < 0.8 {
                    "⣶"
                } else if vals.1 < 1.0 {
                    "⣾"
                } else {
                    panic!(
                        "Value at position {} is out of range: {}",
                        len - (i * 2) + 1,
                        vals.1
                    )
                }
            } else if vals.0 < 1.0 {
                if vals.1 < 0.2 {
                    "⡇"
                } else if vals.1 < 0.4 {
                    "⣇"
                } else if vals.1 < 0.6 {
                    "⣧"
                } else if vals.1 < 0.8 {
                    "⣷"
                } else if vals.1 < 1.0 {
                    "⣿"
                } else {
                    panic!(
                        "Value at position {} is out of range: {}",
                        len - (i * 2) + 1,
                        vals.1
                    )
                }
            } else {
                panic!(
                    "Value at position {} is out of range: {}",
                    len - (i * 1),
                    vals.0
                )
            },
            None,
            None,
        ));
    }

    r
}

fn round(num: f64, decimal_points: u32) -> f64 {
    let mult: f64 = u32::pow(10, decimal_points).into();
    (num * mult as f64).round() / mult as f64
}
