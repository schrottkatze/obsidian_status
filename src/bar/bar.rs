use crate::bar::module::Module;
use crate::Colored;
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

    pub fn renderer(&self) {
        loop {
            let term_size = terminal_size();

            if let Some((Width(w), Height(h))) = term_size {
                let mut unfinished_bar: Vec<UnfinishedBarSeg> = vec![];

                for module in &self.modules {
                    match module {
                        Module::DynSpacer => {
                            unfinished_bar.push(UnfinishedBarSeg::DynSpacer);
                        }
                        Module::Background(bg_mod) => {
                            // unfinished_bar.push(UnfinishedBarSeg::Completed(
                            // match &bg_mod.current {
                            //     None => Colored::from_str(""),
                            //     Some(v) => v.clone(),
                            // },
                            // ));
                        }
                        Module::SingleThreaded(single_threaded_module) => unfinished_bar.push(
                            UnfinishedBarSeg::Completed((single_threaded_module.render_mod)()),
                        ),
                        Module::MultiThreaded(multi_threaded_module) => {
                            unfinished_bar.push(UnfinishedBarSeg::RunningThread(
                                multi_threaded_module.start_render_thread(),
                            ));
                        }
                    }
                }
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
