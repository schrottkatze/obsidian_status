pub mod module;

use crate::bar::module::Module;
use crate::formatting::color::Color;
use crate::formatting::colored::ColoredString;
use crate::{AnsiFormat, Colored};
use std::cell::RefCell;
use std::sync::mpsc::{channel, Sender};
use std::thread;
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
        let (tx, rx) = channel::<()>();
        self.updater(tx.clone());
        for _r in rx {
            self.build_bar(tx.clone());
        }
    }

    fn updater(&self, tx: Sender<()>) -> JoinHandle<()> {
        let interval = self.fixed_interval;
        thread::spawn(move || loop {
            tx.send(()).expect("failed to send");
            sleep(interval)
        })
    }

    fn build_bar(&self, tx: Sender<()>) {
        let term_size = terminal_size();

        if let Some((Width(w), Height(_h))) = term_size {
            self.start_module_threads(tx);

            let builder = BarBuilder::new(&self.modules);
            let r = builder.build_bar(w);

            println!("{}", r);
        }
    }

    fn start_module_threads(&self, tx: Sender<()>) {
        for module in &self.modules {
            match module {
                Module::MultiThreaded(module) => module.start(),
                Module::Background(module) => module.start(tx.clone()),
                _ => {}
            }
        }
    }
}

struct BarBuilder<'a> {
    modules: &'a Vec<Module>,
    bar: RefCell<Vec<UnfinishedBarSeg<'a>>>,
}

impl<'a> BarBuilder<'a> {
    pub fn new(modules: &'a Vec<Module>) -> Self {
        BarBuilder {
            modules,
            bar: RefCell::new(Vec::new()),
        }
    }

    pub fn build_bar(&self, w: u16) -> String {
        self.finish_least_expensive();
        self.join_multithreaded();
        self.assemble_bar(w)
    }

    fn finish_least_expensive(&self) {
        let mut bar_mut = self.bar.borrow_mut();
        for module in self.modules {
            match module {
                Module::Background(module) => {
                    bar_mut.push(UnfinishedBarSeg::Done(module.get_latest()))
                }
                Module::SingleThreaded(module) => {
                    bar_mut.push(UnfinishedBarSeg::Done((module.run)()))
                }
                module => bar_mut.push(UnfinishedBarSeg::Module(module)),
            }
        }
    }

    fn join_multithreaded(&self) {
        for seg in self.bar.borrow_mut().iter_mut() {
            if let UnfinishedBarSeg::Module(Module::MultiThreaded(m)) = seg {
                let r = m.join_running().unwrap();
                *seg = UnfinishedBarSeg::Done(r);
            }
        }
    }

    fn make_spacer(&self, w: u16) -> String {
        let mut content_length: u16 = 0;
        let mut spacer_count: u16 = 0;

        for seg in self.bar.borrow().iter() {
            match seg {
                UnfinishedBarSeg::Done(col) => content_length += col.len_visible_chars() as u16,
                UnfinishedBarSeg::Module(m) => match m {
                    Module::DynSpacer => spacer_count += 1,
                    _ => panic!("unexpected module when counting spacers!"),
                },
            }
        }

        " ".repeat(((w - content_length) / spacer_count) as usize)
    }

    fn assemble_bar(&self, w: u16) -> String {
        let spacer = self.make_spacer(w);

        let mut err_colored = Colored::new();
        err_colored.push_el(ColoredString::new(
            "### ERR ###",
            Some(AnsiFormat::with_fg(Color::Red)),
            None,
        ));

        let mut r = String::new();
        for seg in self.bar.borrow().iter() {
            match seg {
                UnfinishedBarSeg::Done(c) => r.push_str(&*c.to_string()),
                UnfinishedBarSeg::Module(m) => match m {
                    Module::DynSpacer => r.push_str(&spacer),
                    _ => panic!("unexpected non-spacer module when assembling bar!"),
                },
            }
        }

        r
    }
}

enum UnfinishedBarSeg<'a> {
    Module(&'a Module),
    Done(Colored),
}
