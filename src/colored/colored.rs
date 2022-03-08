use super::text_format_conf::{TextFormatConf, Color};
use std::fmt;

pub struct Colored {
    conf: TextFormatConf,
    content: String,
}

impl Colored {
    pub fn new(org_str: &str, initial_conf: TextFormatConf) -> Colored {
        Colored {
            conf: initial_conf,
            content: String::from(org_str),
        }
    }

    pub fn get_plain(&self) -> &str {
        &self.content
    }
    pub fn get_colored(&self) -> String {
        let r = String::new();

        format!(
            "{}{}",
            self.conf.get_ansi_color_code(),
            &self.content,
        )
    }
}

impl fmt::Display for Colored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_colored())
    }
}
