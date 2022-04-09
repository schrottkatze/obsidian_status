use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Range;

use crate::AnsiFormat;

#[derive(Clone)]
pub struct Colored {
    content: Vec<ColoredString>,
}
#[allow(dead_code)]
impl Colored {
    pub fn new() -> Self {
        Colored { content: vec![] }
    }
    pub fn push_el(&mut self, val: ColoredString) {
        self.content.push(val);
    }
    pub fn push_colored(&mut self, val: &mut Colored) {
        self.content.append(&mut val.content)
    }
    pub fn from_str(s: &str) -> Self {
        let mut r = Colored::new();
        r.push_el(ColoredString::new(s, None, None));
        r
    }
    pub fn len_visible_chars(&self) -> usize {
        let mut r: usize = 0;

        for e in &self.content {
            r += e.len_visible_chars()
        }

        r
    }
    pub fn index(&self, index: Range<usize>) -> Self {
        // index, rest
        let mut range_rests = index;
        let mut before = (0, 0);
        let mut after = (0, 0);

        for (_pos, item) in self.content.iter().enumerate() {
            if item.len_visible_chars() >= range_rests.start {
                before.1 = range_rests.start - 1;
            } else {
                range_rests.start -= item.len_visible_chars();
                before.0 += 1;
            }

            if item.len_visible_chars() >= range_rests.end {
                after.1 = range_rests.end - 1;
            } else {
                range_rests.end -= item.len_visible_chars();
                after.0 += 1;
            }
        }

        let mut r = Colored::new();

        if before.0 == after.0 {
            r.push_el(
                self.content[before.0]
                    .index(before.1..after.1)
                    .to_colored_string(),
            );
        } else {
            r.push_el(
                self.content[before.0]
                    .index(before.1..self.content[before.0].len_visible_chars())
                    .to_colored_string(),
            );

            for i in (before.0 + 1)..after.0 {
                r.push_el(self.content[i].clone())
            }

            r.push_el(self.content[after.0].index(0..after.1).to_colored_string());
        }

        r
    }
}
impl Display for Colored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut r = String::new();

        for i in &self.content {
            r.push_str(&i.to_string())
        }

        write!(f, "{}", r)
    }
}

#[derive(Clone)]
pub struct ColoredString {
    content: String,
    before: Option<AnsiFormat>,
    after: Option<AnsiFormat>,
}
impl ColoredString {
    pub fn new(content: &str, before: Option<AnsiFormat>, after: Option<AnsiFormat>) -> Self {
        Self {
            content: content.to_string(),
            before,
            after,
        }
    }
    pub fn index(&self, index: Range<usize>) -> ColoredStringSliceRef {
        ColoredStringSliceRef {
            content: &self.content[index],
            before: match &self.before {
                Some(b) => Some(b),
                _ => None,
            },
            after: match &self.after {
                Some(a) => Some(a),
                _ => None,
            },
        }
    }
    #[inline]
    pub fn len_visible_chars(&self) -> usize {
        self.content.chars().count()
    }
}
impl Display for ColoredString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            match &self.before {
                Some(b) => b.to_string(),
                None => "".to_string(),
            },
            &self.content,
            match &self.after {
                Some(after) => after.to_string(),
                None => "".to_string(),
            }
        )
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct ColoredStringSliceRef<'a> {
    content: &'a str,
    before: Option<&'a AnsiFormat>,
    after: Option<&'a AnsiFormat>,
}
impl Display for ColoredStringSliceRef<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_colored_string())
    }
}
impl ColoredStringSliceRef<'_> {
    pub fn to_colored_string(self) -> ColoredString {
        ColoredString {
            content: self.content.to_string(),
            before: self.before.cloned(),
            after: self.before.cloned(),
        }
    }
}
