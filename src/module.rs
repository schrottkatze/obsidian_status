// Imports {{{
use std::sync::Arc;
use std::thread;

use super::formatting::colored::Colored;
use super::formatting::multi_colored::MultiColored;
use super::formatting::text_format_conf::TextFormatConf;
// }}}

pub struct Module {
    max_len: Arc<usize>,
    content_render: Arc<fn() -> MultiColored>,
    render_condition: Arc<fn() -> bool>,
    content_color: TextFormatConf,
}

impl Module {
    // Initializer {{{
    pub fn new(
        max_len: usize,
        content_render: fn() -> MultiColored,
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
    // }}}

    pub fn start_render_thread(
        &self,
        seps: [&Colored; 2],
    ) -> std::thread::JoinHandle<MultiColored> {
        let content_render = self.content_render.clone();
        let max_len = self.max_len.clone();
        let render_condition = self.render_condition.clone();
        let seps = [seps[0].to_string(), seps[1].to_string()];
        let content_color = self.content_color.clone();

        thread::spawn(move || {
            let mut r = MultiColored::new();

            if render_condition() {
                let mut content: MultiColored = (content_render)();

                if content.len_visible_chars() > *max_len {
                    content = content[0..*max_len + 1];
                }

                r.push_colored(seps[0])
                    .push_colored(content)
                    .push_colored(seps[1])
            } else {
                (String::new(), 0)
            }
        })
    }
}
