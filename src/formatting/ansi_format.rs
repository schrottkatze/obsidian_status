use crate::formatting::color::Color;
use crate::formatting::format_conf::FormatConf;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Clone)]
pub enum AnsiFormat {
    Reset,
    Conf(FormatConf),
}
#[allow(dead_code)]
impl AnsiFormat {
    pub fn with_fg(fg: Color) -> Self {
        Self::Conf(FormatConf::with_fg(fg))
    }
    pub fn with_bg(bg: Color) -> Self {
        Self::Conf(FormatConf::with_bg(bg))
    }
    pub fn empty() -> Self {
        Self::Conf(FormatConf::new())
    }
}
impl Display for AnsiFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Reset => String::from("\x1b[0m"),
                Self::Conf(conf) => conf.to_string(),
            }
        )
    }
}
