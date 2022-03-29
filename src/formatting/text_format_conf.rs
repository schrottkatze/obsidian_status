// Color enum {{{
// }}}

// tfc {{{
#[derive(Clone)]
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

#[allow(dead_code)]
impl TextFormatConf {
    // initializers {{{
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
    // }}}

    pub fn get_ansi_color_code(&self) -> String {
        //{{{
        let mut r = String::from("\x1b[");

        if self.reset_before {
            r.push_str("0m\x1b[");
        }

        if let Some(e) = r.chars().nth(r.len() - 1) {
            if e == ';' {
                r.replace_range(r.len() - 1..r.len(), "m");
            } else {
                r = String::from("");
            }
        }

        r
    } //}}}

    // Setters {{{
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
    // }}}
}
// }}}
