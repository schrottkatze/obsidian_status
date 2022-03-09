use std::thread;
use std::sync::Arc;

use super::formatting::colored::Colored;
use super::formatting::text_format_conf::TextFormatConf;
use super::module::Module;

pub enum Segment {
    DynSpacer,
    StaticSpacer(u16),
    StatusSeg(Vec<Module>, SegSepTypes),
}

pub enum SegSepTypes {
    One(Colored),
    Two(Colored, Colored),
    Three(Colored, Colored, Colored),
}

pub struct Bar {
    segments: Vec<Segment>,
    seps_on_sides: (bool, bool),
}

enum BuildingBlock {
    Dyn,
    Finished(String),
    SegBuilders(Vec<thread::JoinHandle<String>>)
}

impl Bar {
    pub fn new(seps_on_sides: (bool, bool)) -> Bar {
        Bar { segments: vec![], seps_on_sides }
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
                },
                Segment::StatusSeg(mods, seps) => bar_segs_prebuild.push(BuildingBlock::SegBuilders(Bar::start_seg_threads(mods, seps, self.seps_on_sides, i == 0, i == self.segments.len() - 1)))
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
                            Err(_) => String::from("ERR"),
                        };

                        len += module.len() as u16;
                        assembled_status_segs.push(BuildingBlock::Finished(module));
                    }
                },
                BuildingBlock::Finished(v) => assembled_status_segs.push(BuildingBlock::Finished(v)),
                BuildingBlock::Dyn => {
                    assembled_status_segs.push(BuildingBlock::Dyn);
                    dyn_amount += 1;
                },
            }
        }

        let mut final_bar: String = String::new();
        let dyn_built = " ".repeat(((w - len) / dyn_amount) as usize);

        for seg in assembled_status_segs {
            match seg {
                BuildingBlock::Finished(v) => final_bar.push_str(&v),
                BuildingBlock::Dyn => final_bar.push_str(&dyn_built),
                _ => ()
            }
        }

        println!("{}", final_bar);
    }

    fn start_seg_threads(mods: &Vec<Module>, seps: &SegSepTypes, sep_cfg: (bool, bool), first_of_bar: bool, last_of_bar: bool) -> Vec<thread::JoinHandle<String>> {
        let mut r = vec![];

        for (i, module) in mods.iter().enumerate() {
            let seps_mod: [&str; 2] = match seps {
                SegSepTypes::One(sep) => if i == 0 { [sep.get_colored(), sep.get_colored()] } else { ["", sep.get_colored()]},
                SegSepTypes::Two(before, after) => [before.get_colored(), after.get_colored()],
                SegSepTypes::Three(before, mid, after) => [if i == 0 { before.get_colored() } else { "" }, if i == mods.len() - 1 { after.get_colored() } else { mid.get_colored() }],
            };

            r.push(module.start_render_thread(seps_mod));
        }

        r
    }
}
