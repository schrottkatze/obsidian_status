use std::sync::Arc;
use std::thread;

use super::formatting::colored::Colored;

pub struct Module {
    max_len: Arc<usize>,
    content_render: Arc<fn() -> String>,
    render_condition: Arc<fn() -> bool>,
}

impl Module {
    pub fn new(
        max_len: usize,
        content_render: fn() -> String,
        render_condition: Option<fn() -> bool>,
    ) -> Module {
        Module {
            max_len: Arc::new(max_len),
            content_render: Arc::new(content_render),
            render_condition: Arc::new(match render_condition {
                Some(v) => v,
                None => || true,
            }),
        }
    }

    pub fn start_render_thread(&self, seps: [&str; 2]) -> std::thread::JoinHandle<(String, u16)> {
        let content_render = self.content_render.clone();
        let max_len = self.max_len.clone();
        let render_condition = self.render_condition.clone();
        let seps = [seps[0].to_string(), seps[1].to_string()];

        thread::spawn(move || {
            if render_condition() {
                let mut r = (content_render)();

                if r.len() > *max_len {
                    r = r[0..=*max_len].to_string();
                }

                (format!("{}{}{}", seps[0], r, seps[1]), r.len() as u16)
            } else {
                (String::new(), 0)
            }
        })
    }
}

pub enum BarRenderPos {
    FirstOfBar,
    FirstOfSegment,
    LastOfBar,
    LastOfSegment,
}
