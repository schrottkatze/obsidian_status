use std::{io, io::Write, thread};

use super::formatting::colored::Colored;
use super::formatting::text_format_conf::TextFormatConf;
use super::module::Module;

#[allow(dead_code)]
pub enum Segment {
    DynSpacer,
    StaticSpacer(u16),
    StatusSeg(Vec<Module>, SegSepTypes),
}

#[allow(dead_code)]
pub enum SegSepTypes {
    One(Colored),
    Two(Colored, Colored),
    Three(Colored, Colored, Colored),
}

enum BuildingBlock {
    Dyn,
    Finished(String),
    SegBuilders(Vec<thread::JoinHandle<(String, u16)>>),
}

pub struct Bar {
    segments: Vec<Segment>,
}

impl Bar {
    pub fn new() -> Bar {
        Bar {
            segments: vec![],
        }
    }

    pub fn add_segment(&mut self, seg: Segment) -> &mut Bar {
        self.segments.push(seg);
        self
    }

    pub fn render(&self, w: u16) {
        let mut bar_segs_prebuild: Vec<BuildingBlock> = vec![];
        let mut len: u16 = 0;

        for (i, seg) in self.segments.iter().enumerate() {
            match seg {
                Segment::DynSpacer => bar_segs_prebuild.push(BuildingBlock::Dyn),
                Segment::StaticSpacer(space) => {
                    bar_segs_prebuild.push(BuildingBlock::Finished(" ".repeat(*space as usize)));
                    len += space;
                }
                Segment::StatusSeg(mods, seps) => {
                    let (builders, sep_lens) = Bar::start_seg_threads(
                        mods,
                        seps,
                    );
                    len += sep_lens;

                    bar_segs_prebuild.push(BuildingBlock::SegBuilders(builders))
                }
            }
        }

        let mut assembled_status_segs: Vec<BuildingBlock> = vec![];
        let mut dyn_amount = 0;

        for part in bar_segs_prebuild {
            match part {
                BuildingBlock::SegBuilders(handles) => {
                    for handle in handles {
                        let module = match handle.join() {
                            Ok(v) => v,
                            Err(_) => (String::new(), 0),
                        };

                        len += module.1;

                        assembled_status_segs.push(BuildingBlock::Finished(module.0));
                    }
                }
                BuildingBlock::Finished(v) => {
                    assembled_status_segs.push(BuildingBlock::Finished(v))
                }
                BuildingBlock::Dyn => {
                    assembled_status_segs.push(BuildingBlock::Dyn);
                    dyn_amount += 1;
                }
            }
        }

        let mut final_bar: String = String::new();
        let dyn_built = " ".repeat(((w - len) / dyn_amount) as usize);

        for seg in assembled_status_segs {
            match seg {
                BuildingBlock::Finished(v) => final_bar.push_str(&v),
                BuildingBlock::Dyn => final_bar.push_str(&dyn_built),
                _ => (),
            }
        }

        println!("{}", final_bar);
        io::stdout().flush().unwrap();
    }

    fn start_seg_threads(
        mods: &[Module],
        seps: &SegSepTypes,
    ) -> (Vec<thread::JoinHandle<(String, u16)>>, u16) {
        let mut r = vec![];
        let mut sep_lens: u16 = 0;
        let empty_colored = Colored::new("", TextFormatConf::new(), false);

        for (i, module) in mods.iter().enumerate() {
            let seps_mod: [&Colored; 2] = match seps {
                SegSepTypes::One(sep) => {
                    if i == 0 {
                        [sep, sep]
                    } else {
                        [&empty_colored, sep]
                    }
                }
                SegSepTypes::Two(before, after) => [before, after],
                SegSepTypes::Three(before, mid, after) => [
                    if i == 0 { before } else { &empty_colored },
                    if i == mods.len() - 1 { after } else { mid },
                ],
            };

            sep_lens += (seps_mod[0].get_plain().chars().count()
                + seps_mod[1].get_plain().chars().count()) as u16;

            r.push(
                module.start_render_thread([seps_mod[0].get_colored(), seps_mod[1].get_colored()]),
            );
        }

        (r, sep_lens)
    }
}
