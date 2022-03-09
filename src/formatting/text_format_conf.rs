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

pub struct TextFormatConf {
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
    reset_before: bool,
}

impl TextFormatConf {
    pub fn new() -> TextFormatConf {
        TextFormatConf {
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
            reset_before: true,
        }
    }
    pub fn fg_only(fg: Color) -> TextFormatConf {
        let mut r = TextFormatConf::new();

        r.set_fg(Some(fg));

        r
    }
    pub fn bg_only(bg: Color) -> TextFormatConf {
        let mut r = TextFormatConf::new();

        r.set_bg(Some(bg));

        r
    }
    pub fn fg_and_bg(fg: Color, bg: Color) -> TextFormatConf {
        let mut r = TextFormatConf::new();

        r.set_fg(Some(fg)).set_bg(Some(bg));

        r
    }

    pub fn get_ansi_color_code(&self) -> String {
        let mut r = String::from("\x1b[");

        if self.reset_before {
            r.push_str("0m\x1b[");
        }

        if let Some(bold) = self.bold {
            if bold {
                r.push_str("1;");
            } else {
                r.push_str("22;");
            }
        }

        if let Some(dim) = self.dim {
            if dim {
                r.push_str("2;");
            } else {
                r.push_str("22;");
            }
        }

        if let Some(italic) = self.italic {
            if italic {
                r.push_str("3;");
            } else {
                r.push_str("23;");
            }
        }

        if let Some(underline) = self.underline {
            if underline {
                r.push_str("4;");
            } else {
                r.push_str("24;");
            }
        }

        if let Some(blink) = self.blink {
            if blink {
                r.push_str("5;");
            } else {
                r.push_str("25;");
            }
        }

        if let Some(reverse) = self.reverse {
            if reverse {
                r.push_str("7;");
            } else {
                r.push_str("27;");
            }
        }

        if let Some(hidden) = self.hidden {
            if hidden {
                r.push_str("8;");
            } else {
                r.push_str("28;");
            }
        }

        if let Some(strikethrough) = self.strikethrough {
            if strikethrough {
                r.push_str("9;");
            } else {
                r.push_str("29;");
            }
        }

        if let Some(fg) = &self.fg {
            match fg {
                Color::Black => r.push_str("30;"),
                Color::Red => r.push_str("31;"),
                Color::Green => r.push_str("32;"),
                Color::Yellow => r.push_str("33;"),
                Color::Blue => r.push_str("34;"),
                Color::Magenta => r.push_str("35;"),
                Color::Cyan => r.push_str("36;"),
                Color::White => r.push_str("37;"),
                Color::HCBlack => r.push_str("90;"),
                Color::HCRed => r.push_str("91;"),
                Color::HCGreen => r.push_str("92;"),
                Color::HCYellow => r.push_str("93;"),
                Color::HCBlue => r.push_str("94;"),
                Color::HCMagenta => r.push_str("95;"),
                Color::HCCyan => r.push_str("96;"),
                Color::HCWhite => r.push_str("97;"),
                Color::XTerm256(id) => r.push_str(format!("38;5;{};", id).as_str()),
                Color::Rgb((red, green, blue)) => {
                    r.push_str(format!("38;2;{};{};{};", red, green, blue).as_str())
                }
            }
        }

        if let Some(bg) = &self.bg {
            match bg {
                Color::Black => r.push_str("40;"),
                Color::Red => r.push_str("41;"),
                Color::Green => r.push_str("42;"),
                Color::Yellow => r.push_str("43;"),
                Color::Blue => r.push_str("44;"),
                Color::Magenta => r.push_str("45;"),
                Color::Cyan => r.push_str("46;"),
                Color::White => r.push_str("47;"),
                Color::HCBlack => r.push_str("100;"),
                Color::HCRed => r.push_str("101;"),
                Color::HCGreen => r.push_str("102;"),
                Color::HCYellow => r.push_str("103;"),
                Color::HCBlue => r.push_str("104;"),
                Color::HCMagenta => r.push_str("105;"),
                Color::HCCyan => r.push_str("106;"),
                Color::HCWhite => r.push_str("107;"),
                Color::XTerm256(id) => r.push_str(format!("48;5;{};", id).as_str()),
                Color::Rgb((red, green, blue)) => {
                    r.push_str(format!("48;2;{};{};{};", red, green, blue).as_str())
                }
            }
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

    pub fn set_fg(&mut self, col: Option<Color>) -> &mut TextFormatConf {
        self.fg = col;
        self
    }
    pub fn set_bg(&mut self, col: Option<Color>) -> &mut TextFormatConf {
        self.bg = col;
        self
    }
    pub fn set_bold(&mut self, bold: Option<bool>) -> &mut TextFormatConf {
        self.bold = bold;
        self
    }
    pub fn set_dim(&mut self, dim: Option<bool>) -> &mut TextFormatConf {
        self.dim = dim;
        self
    }
    pub fn set_italic(&mut self, italic: Option<bool>) -> &mut TextFormatConf {
        self.italic = italic;
        self
    }
    pub fn set_underline(&mut self, underline: Option<bool>) -> &mut TextFormatConf {
        self.underline = underline;
        self
    }
    pub fn set_blink(&mut self, blink: Option<bool>) -> &mut TextFormatConf {
        self.blink = blink;
        self
    }
    pub fn set_reverse(&mut self, reverse: Option<bool>) -> &mut TextFormatConf {
        self.reverse = reverse;
        self
    }
    pub fn set_hidden(&mut self, hidden: Option<bool>) -> &mut TextFormatConf {
        self.hidden = hidden;
        self
    }
    pub fn set_strikethrough(&mut self, strikethrough: Option<bool>) -> &mut TextFormatConf {
        self.strikethrough = strikethrough;
        self
    }
    pub fn set_reset(&mut self, reset: bool) -> &mut TextFormatConf {
        self.reset_before = reset;
        self
    }
}
