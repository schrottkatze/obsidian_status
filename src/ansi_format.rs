use std::fmt::{write, Display, Formatter};
use termcolor::Ansi;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    HCBlack,
    HCRed,
    HCGreen,
    HCYellow,
    HCBlue,
    HCMagenta,
    HCCyan,
    HCWhite,
    XTerm256(u8),
    Rgb((u8, u8, u8)),
}

impl Color {
    pub fn get_ansi_fg(&self) -> String {
        let mut r = String::from("\x1b[");

        r += &self.get_ansi_fg_part();
        r.replace_range(r.len() - 1..r.len(), "m");

        r
    }
    pub fn get_ansi_fg_part(&self) -> String {
        match self {
            Color::Black => "30;".to_string(),
            Color::Red => "31;".to_string(),
            Color::Green => "32;".to_string(),
            Color::Yellow => "33;".to_string(),
            Color::Blue => "34;".to_string(),
            Color::Magenta => "35;".to_string(),
            Color::Cyan => "36;".to_string(),
            Color::White => "37;".to_string(),
            Color::HCBlack => "90;".to_string(),
            Color::HCRed => "91;".to_string(),
            Color::HCGreen => "92;".to_string(),
            Color::HCYellow => "93;".to_string(),
            Color::HCBlue => "94;".to_string(),
            Color::HCMagenta => "95;".to_string(),
            Color::HCCyan => "96;".to_string(),
            Color::HCWhite => "97;".to_string(),
            Color::XTerm256(id) => format!("38;5;{};", id),
            Color::Rgb((r, g, b)) => format!("38;2;{};{};{};", r, g, b),
        }
    }

    pub fn get_ansi_bg(&self) -> String {
        let mut r = String::from("\x1b[");

        r += &self.get_ansi_fg_part();
        r.replace_range(r.len() - 1..r.len(), "m");

        r
    }
    pub fn get_ansi_bg_part(&self) -> String {
        match self {
            Color::Black => "40;".to_string(),
            Color::Red => "41;".to_string(),
            Color::Green => "42;".to_string(),
            Color::Yellow => "43;".to_string(),
            Color::Blue => "44;".to_string(),
            Color::Magenta => "45;".to_string(),
            Color::Cyan => "46;".to_string(),
            Color::White => "47;".to_string(),
            Color::HCBlack => "100;".to_string(),
            Color::HCRed => "101;".to_string(),
            Color::HCGreen => "102;".to_string(),
            Color::HCYellow => "103;".to_string(),
            Color::HCBlue => "104;".to_string(),
            Color::HCMagenta => "105;".to_string(),
            Color::HCCyan => "106;".to_string(),
            Color::HCWhite => "107;".to_string(),
            Color::XTerm256(id) => format!("48;5;{};", id),
            Color::Rgb((r, g, blue)) => format!("48;2;{};{};{};", r, g, blue),
        }
    }
}

#[derive(Clone)]
pub struct NoResetFormatConf {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: Option<bool>,
    dim: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    blink: Option<bool>,
    reverse: Option<bool>,
    hidden: Option<bool>,
    strikethrough: Option<bool>,
}

impl NoResetFormatConf {
    pub fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: None,
            dim: None,
            italic: None,
            underline: None,
            blink: None,
            reverse: None,
            hidden: None,
            strikethrough: None,
        }
    }
    pub fn with_fg(fg: Color) -> Self {
        let mut r = Self::new();

        r.set_fg(Some(fg));

        r
    }
    pub fn with_bg(bg: Color) -> Self {
        let mut r = Self::new();

        r.set_bg(Some(bg));

        r
    }

    pub fn get_code(&self) -> String {
        let mut r = String::from("\x1b[");

        if let Some(bold) = self.bold {
            if bold {
                r.push_str("0;");
            } else {
                r.push_str("21;");
            }
        }

        if let Some(dim) = self.dim {
            if dim {
                r.push_str("1;");
            } else {
                r.push_str("21;");
            }
        }

        if let Some(italic) = self.italic {
            if italic {
                r.push_str("2;");
            } else {
                r.push_str("22;");
            }
        }

        if let Some(underline) = self.underline {
            if underline {
                r.push_str("3;");
            } else {
                r.push_str("23;");
            }
        }

        if let Some(blink) = self.blink {
            if blink {
                r.push_str("4;");
            } else {
                r.push_str("24;");
            }
        }

        if let Some(reverse) = self.reverse {
            if reverse {
                r.push_str("6;");
            } else {
                r.push_str("26;");
            }
        }

        if let Some(hidden) = self.hidden {
            if hidden {
                r.push_str("7;");
            } else {
                r.push_str("27;");
            }
        }

        if let Some(strikethrough) = self.strikethrough {
            if strikethrough {
                r.push_str("8;");
            } else {
                r.push_str("28;");
            }
        }

        if let Some(fg) = &self.fg {
            r.push_str(&fg.get_ansi_fg_part());
        }

        if let Some(bg) = &self.bg {
            r.push_str(&bg.get_ansi_bg_part());
        }

        if let Some(e) = r.chars().nth(r.len() - 1) {
            if e == ';' {
                r.replace_range(r.len() - 1..r.len(), "m");
            } else {
                r = String::from("");
            }
        }

        r
    }

    pub fn set_fg(&mut self, fg: Option<Color>) {
        self.fg = fg;
    }
    pub fn set_bg(&mut self, bg: Option<Color>) {
        self.bg = bg;
    }
    pub fn set_bold(&mut self, bold: Option<bool>) {
        self.bold = bold;
    }
    pub fn set_dim(&mut self, dim: Option<bool>) {
        self.dim = dim;
    }
    pub fn set_italic(&mut self, italic: Option<bool>) {
        self.italic = italic;
    }
    pub fn set_underline(&mut self, underline: Option<bool>) {
        self.underline = underline;
    }
    pub fn set_blink(&mut self, blink: Option<bool>) {
        self.blink = blink;
    }
    pub fn set_reverse(&mut self, reverse: Option<bool>) {
        self.reverse = reverse;
    }
    pub fn set_hidden(&mut self, hidden: Option<bool>) {
        self.hidden = hidden;
    }
    pub fn set_strikethrough(&mut self, strikethrough: Option<bool>) {
        self.strikethrough = strikethrough;
    }
}

impl Display for NoResetFormatConf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_code())
    }
}

pub enum AnsiFormat {
    Reset,
    Conf(NoResetFormatConf),
}

impl AnsiFormat {
    pub fn with_fg(fg: Color) -> Self {
        Self::Conf(NoResetFormatConf::with_fg(fg))
    }
    pub fn with_bg(bg: Color) -> Self {
        Self::Conf(NoResetFormatConf::with_bg(bg))
    }

    pub fn get_code(&self) -> String {
        match self {
            AnsiFormat::Reset => String::from("\x1b[0m"),
            AnsiFormat::Conf(c) => c.get_code(),
        }
    }
}

impl Display for AnsiFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Self::Reset => String::from("\x1b[0m"),
            Self::Conf(conf) => conf.get_code(),
        };

        write!(f, "{}", code)
    }
}
