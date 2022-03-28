// Imports {{{
use super::text_format_conf::TextFormatConf;
use std::fmt;
use std::ops;
// }}}

// ideas:
// - get_colored method instead
// - store content and tfc seperately
pub struct Colored {
    plain: String,
    tfc: TextFormatConf,
    reset_after: bool,
}

impl Colored {
    pub fn new(org_str: &str, initial_conf: TextFormatConf, reset_after: bool) -> Colored {
        Colored {
            plain: String::from(org_str),
            tfc: initial_conf,
            reset_after,
        }
    }

    pub fn get_plain(&self) -> &str {
        &self.plain
    }

    pub fn get_colored(&self) -> String {
        format!(
            "{}{}{}",
            self.tfc.get_ansi_color_code(),
            self.plain,
            if self.reset_after { "\x1b[0m" } else { "" }
        )
    }

    pub fn get_colored_part(&self, start: usize, end: usize) -> Option<Colored> {
        if end < self.get_plain().len() {
            Some(Colored {
                plain: String::from(&self.plain[start..end]),
                tfc: self.tfc,
                reset_after: self.reset_after,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Colored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_colored())
    }
}

impl ops::Index<ops::Range<usize>> for Colored {
    type Output = Colored;

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &Colored {
            plain: String::from(&self.plain[index]),
            tfc: self.tfc,
            reset_after: self.reset_after,
        }
    }
}
