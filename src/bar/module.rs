use crate::Colored;
use std::ops::{Deref, DerefMut};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;

pub enum Module {
    DynSpacer,
    Background(Background),
    SingleThreaded(SingleThreaded),
    MultiThreaded(MultiThreaded),
}
impl Module {
    pub fn new_single_threaded(run: fn() -> Colored) -> Module {
        Module::SingleThreaded(SingleThreaded::new(run))
    }
    pub fn new_multi_threaded(
        run: fn(prev: Option<Vec<Colored>>) -> Colored,
        collect_prev_data: bool,
    ) -> Module {
        Module::MultiThreaded(MultiThreaded::new(run, collect_prev_data))
    }
    pub fn new_background(run: fn(rx: mpsc::Sender<Colored>)) -> Module {
        Module::Background(Background::new(run))
    }
}

pub struct SingleThreaded {
    pub run: fn() -> Colored,
}
impl SingleThreaded {
    pub fn new(run: fn() -> Colored) -> Self {
        Self { run }
    }
}

pub struct Background {
    background_thread: JoinHandle<()>,
    rx: mpsc::Receiver<Colored>,
}
impl Background {
    pub fn new(run: fn(rx: mpsc::Sender<Colored>)) -> Self {
        let (tx, rx) = mpsc::channel::<Colored>();
        Background {
            background_thread: thread::spawn(move || run(tx)),
            rx,
        }
    }
}

pub struct MultiThreaded {
    run: Arc<fn(prev: Option<Vec<Colored>>) -> Colored>,
    prev_data: Arc<Mutex<Option<Vec<Colored>>>>,
}

impl MultiThreaded {
    pub fn new(run: fn(prev: Option<Vec<Colored>>) -> Colored, collect_prev_data: bool) -> Self {
        MultiThreaded {
            run: Arc::new(run),
            prev_data: Arc::new(Mutex::new(if collect_prev_data {
                Some(Vec::<Colored>::new())
            } else {
                None
            })),
        }
    }

    pub fn start_render_thread(&self) -> JoinHandle<Colored> {
        let render = self.run.clone();
        let prev = self.prev_data.clone();
        thread::spawn(move || {
            let mut prv = prev.lock().unwrap();
            let r = render(prv.clone());

            if let Some(prv) = prv.deref_mut() {
                prv.push(r.clone())
            }

            r
        })
    }
}
