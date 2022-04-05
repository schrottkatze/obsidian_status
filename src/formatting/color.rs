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
#[allow(dead_code)]
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
