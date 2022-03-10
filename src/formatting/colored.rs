use super::text_format_conf::TextFormatConf;
use std::fmt;

pub struct Colored {
    content: String,
    colored: String,
    conf: TextFormatConf,
}

impl Colored {
    pub fn new(org_str: &str, initial_conf: TextFormatConf) -> Colored {
        Colored {
            colored: Colored::make_colored(org_str, &initial_conf),
            content: String::from(org_str),
            conf: initial_conf,
        }
    }

    fn make_colored(org_str: &str, tfc: &TextFormatConf) -> String {
        format!("{}{}", tfc.get_ansi_color_code(), org_str,)
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
