use super::bar::{BarConfig, SepDuo, SepSet};

pub struct Module {
    max_len: u16,
    content_render: fn() -> String,
    render_condition: fn() -> bool,
}

impl Module {
    pub fn new(max_len: u16, content_render: fn() -> String, render_condition: Option<fn() -> bool>) -> Module {
        Module {
            max_len,
            content_render,
            render_condition: match render_condition {
                Some(v) => v,
                None => || true,
            }
        }
    }

    pub fn render_content(&self) {
    }
}

pub struct RendererPosInfo {
    pub first_of_segment: bool,
    pub last_of_segment: bool,
}
