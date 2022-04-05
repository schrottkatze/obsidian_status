use crate::formatting::color::Color;
use std::fmt::{Display, Formatter, Write};

#[allow(dead_code)]
#[derive(Clone)]
pub struct FormatConf {
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
#[allow(dead_code)]
impl FormatConf {
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
impl Display for FormatConf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut r = String::new();
        write!(&mut r, "\x1b[")?;

        if let Some(bold) = self.bold {
            if bold {
                write!(&mut r, "0;")?;
            } else {
                write!(&mut r, "21;")?;
            }
        }

        if let Some(dim) = self.dim {
            if dim {
                write!(&mut r, "1;")?;
            } else {
                write!(&mut r, "21;")?;
            }
        }

        if let Some(italic) = self.italic {
            if italic {
                write!(&mut r, "2;")?;
            } else {
                write!(&mut r, "22;")?;
            }
        }

        if let Some(underline) = self.underline {
            if underline {
                write!(&mut r, "3;")?;
            } else {
                write!(&mut r, "23;")?;
            }
        }

        if let Some(blink) = self.blink {
            if blink {
                write!(&mut r, "4;")?;
            } else {
                write!(&mut r, "24;")?;
            }
        }

        if let Some(reverse) = self.reverse {
            if reverse {
                write!(&mut r, "6;")?;
            } else {
                write!(&mut r, "26;")?;
            }
        }

        if let Some(hidden) = self.hidden {
            if hidden {
                write!(&mut r, "7;")?;
            } else {
                write!(&mut r, "27;")?;
            }
        }

        if let Some(strikethrough) = self.strikethrough {
            if strikethrough {
                write!(&mut r, "8;")?;
            } else {
                write!(&mut r, "28;")?;
            }
        }

        if let Some(fg) = &self.fg {
            write!(r, "{}", &fg.get_ansi_fg_part())?;
        }

        if let Some(bg) = &self.bg {
            write!(r, "{}", &bg.get_ansi_fg_part())?;
        }

        if let Some(e) = r.chars().nth(r.len() - 1) {
            if e == ';' {
                r.replace_range(r.len() - 1..r.len(), "m");
            } else {
                r = String::from("");
            }
        }

        write!(f, "{}", r)
    }
}
