use super::text_format_conf::TextFormatConf;
use std::fmt;

pub struct Colored {
    content: String,
    colored: String,
}

impl Colored {
    pub fn new(org_str: &str, initial_conf: TextFormatConf, reset_after: bool) -> Colored {
        Colored {
            colored: Colored::make_colored(org_str, &initial_conf, reset_after),
            content: String::from(org_str),
        }
    }

    fn make_colored(org_str: &str, tfc: &TextFormatConf, reset_after: bool) -> String {
        format!(
            "{}{}{}",
            tfc.get_ansi_color_code(),
            org_str,
            if reset_after { "\x1b[0m" } else { "" }
        )
    }

    pub fn get_plain(&self) -> &str {
        &self.content
    }

    pub fn get_colored(&self) -> &str {
        &self.colored
    }
}

impl fmt::Display for Colored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_colored())
    }
}
