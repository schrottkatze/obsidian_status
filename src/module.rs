use std::sync::Arc;
use std::thread;

use super::formatting::colored::Colored;
use super::formatting::text_format_conf::TextFormatConf;

pub struct Module {
    max_len: Arc<usize>,
    content_render: Arc<fn() -> String>,
    render_condition: Arc<fn() -> bool>,
    content_color: TextFormatConf,
}

impl Module {
    pub fn new(
        max_len: usize,
        content_render: fn() -> String,
        content_color: Option<TextFormatConf>,
        render_condition: Option<fn() -> bool>,
    ) -> Module {
        Module {
            max_len: Arc::new(max_len),
            content_render: Arc::new(content_render),
            render_condition: Arc::new(match render_condition {
                Some(v) => v,
                None => || true,
            }),
            content_color: match content_color {
                Some(v) => v,
                None => TextFormatConf::new(),
            },
        }
    }

    pub fn start_render_thread(&self, seps: [&str; 2]) -> std::thread::JoinHandle<(String, u16)> {
        let content_render = self.content_render.clone();
        let max_len = self.max_len.clone();
        let render_condition = self.render_condition.clone();
        let seps = [seps[0].to_string(), seps[1].to_string()];
        let content_color = self.content_color.clone();

        thread::spawn(move || {
            if render_condition() {
                let mut r = (content_render)();

                if r.chars().count() > *max_len {
                    r = r[0..=*max_len].to_string();
                }

                (
                    format!(
                        "{}{}{}",
                        seps[0],
                        Colored::new(&r, content_color, true).get_colored(),
                        seps[1]
                    ),
                    r.chars().count() as u16,
                )
            } else {
                (String::new(), 0)
            }
        })
    }
}
