use std::io::Write;
use termcolor::{BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};

use super::module::{JustifyModule, Module, RendererPosInfo};

// seperators used
// examples under assumption that outer_sep_config is (true, true) and that
// show_both_seps_on_overlap is true as well
pub type SepDuo = (String, String);
pub enum SepSet {
    // "|" => |item|item|
    SingleAlways(String),
    // "<", ">" => <left><left>  <center>  <right><right>
    DualAlways(SepDuo),
    // ">", "|", "<" => >left>left>  |center|  <right<right<
    AlignmentBound(String, String, String),
    // ">", ("(", ")"), "<" => >left>left> (center) <right<right<
    SingleSidesDualCenter(String, SepDuo, String),
    // ("(", ")"), ("[", "]"), ("{", "}") => (left)(left)  [center] {right}{right}
    DualDifferentAll(SepDuo, SepDuo, SepDuo),
}

pub struct Bar {
    modules_r: Vec<Module>,
    modules_l: Vec<Module>,
    modules_c: Vec<Module>,
    config: BarConfig,
}

pub struct BarConfig {
    pub sep_set: SepSet,
    pub color_spec: ColorSpec,
    pub sep_color_spec: ColorSpec,
    pub show_both_seps_on_overlap: bool,
    pub outer_sep_config: (bool, bool),
}

impl Bar {
    pub fn new(config: BarConfig) -> Bar {
        Bar {
            modules_r: vec![],
            modules_l: vec![],
            modules_c: vec![],
            config,
        }
    }

    pub fn render(&self, width: u16) {
        let mut bufstr = BufferedStandardStream::stdout(ColorChoice::Always);
        let mut currentLength = 0;

        //bufstr
        //.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(127, 0, 255))))
        //.unwrap();
        //write!(&mut bufstr, "test").unwrap();
        //bufstr
        //.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 127, 255))))
        //.unwrap();
        //write!(&mut bufstr, "test").unwrap();
        self.modules_l[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Left,
                first_of_block: true,
                last_of_block: false,
            },
            &mut currentLength,
        );
        self.modules_l[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Left,
                first_of_block: false,
                last_of_block: false,
            },
            &mut currentLength,
        );
        self.modules_l[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Left,
                first_of_block: false,
                last_of_block: true,
            },
            &mut currentLength,
        );

        write!(&mut bufstr, "     ");
        self.modules_c[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Center,
                first_of_block: true,
                last_of_block: true,
            },
            &mut currentLength,
        );

        write!(&mut bufstr, "     ");
        self.modules_r[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Right,
                first_of_block: true,
                last_of_block: false,
            },
            &mut currentLength,
        );
        self.modules_r[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Right,
                first_of_block: false,
                last_of_block: false,
            },
            &mut currentLength,
        );
        self.modules_r[0].render_mod(
            &mut bufstr,
            &self.config,
            &RendererPosInfo {
                in_block: JustifyModule::Right,
                first_of_block: false,
                last_of_block: true,
            },
            &mut currentLength,
        );

        write!(&mut bufstr, "\n");
        bufstr.flush();
    }

    pub fn add_module(&mut self, alignment: JustifyModule, module: Module) {
        match alignment {
            JustifyModule::Right => self.modules_r.push(module),
            JustifyModule::Left => self.modules_l.push(module),
            JustifyModule::Center => self.modules_c.push(module),
        }
    }
}
