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

        let spacer_sizes = self.sim_render(width);

        for (i, module) in self.modules_l.iter().enumerate() {
            module.render_mod(
                &mut bufstr,
                &self.config,
                &RendererPosInfo {
                    in_block: JustifyModule::Left,
                    first_of_block: i == 0,
                    last_of_block: i == self.modules_l.len() - 1,
                },
                &mut currentLength,
            );
        }

        write!(&mut bufstr, "{}", " ".repeat(spacer_sizes.0 as usize));

        for (i, module) in self.modules_c.iter().enumerate() {
            module.render_mod(
                &mut bufstr,
                &self.config,
                &RendererPosInfo {
                    in_block: JustifyModule::Center,
                    first_of_block: i == 0,
                    last_of_block: i == self.modules_c.len() - 1,
                },
                &mut currentLength,
            );
        }

        write!(&mut bufstr, "{}", " ".repeat(spacer_sizes.1 as usize));

        for (i, module) in self.modules_r.iter().enumerate() {
            module.render_mod(
                &mut bufstr,
                &self.config,
                &RendererPosInfo {
                    in_block: JustifyModule::Right,
                    first_of_block: i == 0,
                    last_of_block: i == self.modules_r.len() - 1,
                },
                &mut currentLength,
            );
        }

        write!(&mut bufstr, "\r");
        bufstr.flush().unwrap();
    }

    fn sim_render(&self, line_length: u16) -> (u16, u16) {
        let mut modules_l_size: u16 = 0;
        for (i, module) in self.modules_l.iter().enumerate() {
            modules_l_size += module.calcLenStatic(
                &self.config,
                &RendererPosInfo {
                    in_block: JustifyModule::Left,
                    first_of_block: i == 0,
                    last_of_block: i == self.modules_l.len() - 1,
                },
            );
        }

        let mut modules_c_size: u16 = 0;
        for (i, module) in self.modules_c.iter().enumerate() {
            modules_c_size += module.calcLenStatic(
                &self.config,
                &RendererPosInfo {
                    in_block: JustifyModule::Center,
                    first_of_block: i == 0,
                    last_of_block: i == self.modules_c.len() - 1,
                },
            );
        }

        let mut modules_r_size: u16 = 0;
        for (i, module) in self.modules_r.iter().enumerate() {
            modules_r_size += module.calcLenStatic(
                &self.config,
                &RendererPosInfo {
                    in_block: JustifyModule::Right,
                    first_of_block: i == 0,
                    last_of_block: i == self.modules_r.len() - 1,
                },
            );
        }

        let all_mod_size = modules_l_size + modules_c_size + modules_r_size;

        if all_mod_size < line_length {
            (
                (line_length as f32 / 2.0).round() as u16
                    - (modules_l_size + (modules_c_size as f32 / 2.0).round() as u16),
                (line_length as f32 / 2.0).round() as u16
                    - (modules_r_size + (modules_c_size as f32 / 2.0).round() as u16),
            )
        } else {
            (0, 0)
        }
    }

    pub fn add_module(&mut self, alignment: JustifyModule, module: Module) {
        match alignment {
            JustifyModule::Right => self.modules_r.push(module),
            JustifyModule::Left => self.modules_l.push(module),
            JustifyModule::Center => self.modules_c.push(module),
        }
    }
}
