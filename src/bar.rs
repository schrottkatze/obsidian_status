use super::colored::colored::Colored;
use super::colored::text_format_conf{TextFormatConf, Color}
use super::module::Module;

pub enum Segment {
    DynSpacer,
    StaticSpacer(u16),
    SegmentTwoSeps(Vec<Module>, (Colored, Colored)),
    SegmentThreeSeps(Vec<Module>, (Colored, Colored, Colored)),
}

pub struct Bar {
    bar_contents: Vec<Segment>,
}

impl Bar {
    pub fn new() -> Bar {
        let r = Bar {
            bar_contents: vec![],
        };

        r.add_segment(Segment::SegmentTwoSeps(
            vec![],
            (
                Colored::new("<", Some(TextFormatConf::fg_only(Color::Rgb((127, 0, 255))))),
                Colored::new(">", Some(TextFormatConf::fg_only(Color::Rgb((0, 255, 0))))),
        )));

        r.add_segment(Segment::DynSpacer);

        r.add_segment(Segment::SegmentThreeSeps(
            vec![],
            (
                Colored::new("(", Some(TextFormatConf::fg_only(Color::Rgb((127, 0, 255))))),
                Colored::new("|", Some(TextFormatConf::fg_only(Color::Rgb((0, 127, 255))))),
                Colored::new(")", Some(TextFormatConf::fg_only(Color::Rgb((127, 0, 255))))),
        )));

        r
    }

    pub fn add_segment(&mut self, seg: Segment) -> &mut Bar {
        self.bar_contents.push(seg);
        self
    }
}
