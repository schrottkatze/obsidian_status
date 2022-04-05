use crate::bar::module::Module;
use crate::formatting::color::Color;
use crate::formatting::colored::ColoredString;
use crate::{AnsiFormat, Colored};
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};

pub struct Bar {
    modules: Vec<Module>,
    fixed_interval: Duration,
}

impl Bar {
    pub fn new(interval: Duration) -> Self {
        Bar {
            modules: Vec::new(),
            fixed_interval: interval,
        }
    }

    pub fn push_module(&mut self, module: Module) -> &mut Self {
        self.modules.push(module);
        self
    }

    pub fn run(&self) {
        loop {
            let term_size = terminal_size();

            if let Some((Width(w), Height(_h))) = term_size {
                let mut unfinished_bar: Vec<UnfinishedBarSeg> = vec![];

                for module in &self.modules {
                    match module {
                        Module::DynSpacer => {
                            unfinished_bar.push(UnfinishedBarSeg::DynSpacer);
                        }
                        Module::Background(_bg_mod) => {
                            // unfinished_bar.push(UnfinishedBarSeg::Completed(
                            // match &bg_mod.current {
                            //     None => Colored::from_str(""),
                            //     Some(v) => v.clone(),
                            // },
                            // ));
                            todo!()
                        }
                        Module::SingleThreaded(single_threaded_module) => unfinished_bar
                            .push(UnfinishedBarSeg::Completed((single_threaded_module.run)())),
                        Module::MultiThreaded(multi_threaded_module) => {
                            unfinished_bar.push(UnfinishedBarSeg::RunningThread(
                                multi_threaded_module.start_render_thread(),
                            ));
                        }
                    }
                }

                let mut err_colored = Colored::new();
                err_colored.push_el(ColoredString::new(
                    "### ERR ###",
                    Some(AnsiFormat::with_fg(Color::Red)),
                    None,
                ));

                let mut content_length: u16 = 0;
                let mut spacer_count: u16 = 0;

                let mut temp_bar = Vec::<UnfinishedBarSeg>::new();
                for seg in unfinished_bar {
                    match seg {
                        UnfinishedBarSeg::DynSpacer => {
                            spacer_count += 1;
                            temp_bar.push(UnfinishedBarSeg::DynSpacer)
                        }
                        UnfinishedBarSeg::Completed(content) => {
                            content_length += content.len_visible_chars() as u16;
                            temp_bar.push(UnfinishedBarSeg::Completed(content))
                        }
                        UnfinishedBarSeg::RunningThread(handle) => {
                            let joined = handle.join();
                            let r = match joined {
                                Ok(v) => v,
                                Err(e) => err_colored.clone(),
                            };
                            content_length += r.len_visible_chars() as u16;
                            temp_bar.push(UnfinishedBarSeg::Completed(r));
                        }
                    }
                }

                let spacer = " ".repeat(((w - content_length) / spacer_count) as usize);
                let mut r = String::new();
                for item in temp_bar {
                    match item {
                        UnfinishedBarSeg::DynSpacer => r.push_str(&spacer),
                        UnfinishedBarSeg::Completed(colored) => r.push_str(&colored.to_string()),
                        UnfinishedBarSeg::RunningThread(_) => {}
                    }
                }

                println!("{}", r);
            }

            sleep(self.fixed_interval);
        }
    }
}

enum UnfinishedBarSeg {
    DynSpacer,
    Completed(Colored),
    RunningThread(JoinHandle<Colored>),
}
