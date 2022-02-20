use super::bar::{BarConfig, SepDuo, SepSet};
use std::io::Write;
use termcolor::{BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};

pub struct Module {
    module_length: usize,
    content_render: fn() -> String,
    render_condition: Option<fn() -> bool>,
    color: Option<ColorSpec>,
    sep_color: Option<ColorSpec>,
}

#[derive(PartialEq)]
pub enum JustifyModule {
    Left,
    Right,
    Center,
}

impl Module {
    pub fn new(
        module_length: usize,
        content_render: fn() -> String,
        render_condition: Option<fn() -> bool>,
        color: Option<ColorSpec>,
        sep_color: Option<ColorSpec>,
    ) -> Module {
        Module {
            module_length,
            content_render,
            render_condition,
            color,
            sep_color,
        }
    }

    pub fn render_mod(
        &self,
        bufstr: &mut BufferedStandardStream,
        bar_cfg: &BarConfig,
        pos_info: &RendererPosInfo,
        bar_length: &mut u16,
    ) {
        if match self.render_condition {
            Some(f) => f(),
            None => true,
        } {
            self.render_sep(bufstr, bar_cfg, pos_info, true, bar_length);

            let mut content: String = (self.content_render)();
            if content.len() > self.module_length {
                content = String::from(&content[0..self.module_length]);
            }

            bufstr
                .set_color(match &self.color {
                    Some(v) => &v,
                    None => &bar_cfg.color_spec,
                })
                .unwrap();

            let whitespaces = match pos_info.in_block {
                JustifyModule::Left => (self.module_length - content.len(), 0),
                JustifyModule::Right => (0, self.module_length - content.len()),
                JustifyModule::Center => {
                    let half: f32 = (self.module_length - content.len()) as f32 / 2 as f32;
                    (half.ceil() as usize, half.floor() as usize)
                }
            };

            *bar_length += content.len() as u16;
            write!(
                bufstr,
                "{}{}{}",
                " ".repeat(whitespaces.1),
                content,
                " ".repeat(whitespaces.0)
            )
            .unwrap();

            self.render_sep(bufstr, bar_cfg, pos_info, false, bar_length);
        }
    }

    fn render_sep(
        &self,
        bufstr: &mut BufferedStandardStream,
        bar_cfg: &BarConfig,
        pos_info: &RendererPosInfo,
        first_in_mod: bool,
        bar_length: &mut u16,
    ) {
        let mut render = false;

        if pos_info.in_block == JustifyModule::Left {
            if pos_info.first_of_block && first_in_mod && bar_cfg.outer_sep_config.0 {
                render = true
            } else if !first_in_mod {
                render = true
            } else if first_in_mod && !pos_info.first_of_block && bar_cfg.show_both_seps_on_overlap
            {
                render = true
            }
        } else if pos_info.in_block == JustifyModule::Center {
            render = true
        } else if pos_info.in_block == JustifyModule::Right {
            if pos_info.last_of_block && !first_in_mod && bar_cfg.outer_sep_config.1 {
                render = true
            } else if first_in_mod {
                render = true
            } else if !first_in_mod && !pos_info.last_of_block && bar_cfg.show_both_seps_on_overlap
            {
                render = true
            }
        }

        if !render {
            return;
        }

        bufstr
            .set_color(match &self.sep_color {
                Some(v) => &v,
                None => &bar_cfg.sep_color_spec,
            })
            .unwrap();

        let sep = Module::get_correct_sep(bar_cfg, pos_info, first_in_mod);

        *bar_length += sep.len() as u16;

        write!(bufstr, "{}", sep);
    }

    pub fn calcLenStatic(&self, bar_cfg: &BarConfig, pos_info: &RendererPosInfo) -> u16 {
        if (match self.render_condition {
            Some(f) => f,
            None => || true,
        })() {
            let sep0_len: u16 = Module::get_correct_sep(bar_cfg, pos_info, true)
                .len()
                .try_into()
                .unwrap();
            let sep1_len: u16 = Module::get_correct_sep(bar_cfg, pos_info, false)
                .len()
                .try_into()
                .unwrap();

            sep0_len + sep1_len + self.module_length as u16
        } else {
            0
        }
    }

    fn get_correct_sep<'a>(
        bar_cfg: &'a BarConfig,
        pos_info: &RendererPosInfo,
        first_in_mod: bool,
    ) -> &'a str {
        match &bar_cfg.sep_set {
            SepSet::SingleAlways(v) => v,
            SepSet::DualAlways(v) => getCorrectSepFromDuo(&v, first_in_mod),
            SepSet::AlignmentBound(l, c, r) => match pos_info.in_block {
                JustifyModule::Left => l,
                JustifyModule::Right => c,
                JustifyModule::Center => r,
            },
            SepSet::SingleSidesDualCenter(l, c, r) => match pos_info.in_block {
                JustifyModule::Left => l,
                JustifyModule::Right => r,
                JustifyModule::Center => getCorrectSepFromDuo(c, first_in_mod),
            },
            SepSet::DualDifferentAll(l, c, r) => match pos_info.in_block {
                JustifyModule::Left => getCorrectSepFromDuo(l, first_in_mod),
                JustifyModule::Right => getCorrectSepFromDuo(r, first_in_mod),
                JustifyModule::Center => getCorrectSepFromDuo(c, first_in_mod),
            },
        }
    }
}

fn getCorrectSepFromDuo<'a>(duo: &'a SepDuo, first_in_mod: bool) -> &'a String {
    if first_in_mod {
        &duo.0
    } else {
        &duo.1
    }
}

pub struct RendererPosInfo {
    pub in_block: JustifyModule,
    pub first_of_block: bool,
    pub last_of_block: bool,
}
