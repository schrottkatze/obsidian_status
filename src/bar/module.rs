use crate::{Bar, Colored};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub enum Module {
    DynSpacer,
    Background(Background),
    SingleThreaded(SingleThreaded),
    MultiThreaded(MultiThreaded),
}

impl Module {
    pub fn new_single_threaded(render_mod: fn() -> Colored) -> Module {
        Module::SingleThreaded(SingleThreaded::new(render_mod))
    }
    pub fn new_multi_threaded(render_fn: fn(prev: Vec<Colored>) -> Colored) -> Module {
        Module::MultiThreaded(MultiThreaded::new(render_fn))
    }
    pub fn new_background(render: fn(rx: mpsc::Sender<Colored>)) -> Module {
        Module::Background(Background::new(render))
    }
}

pub struct SingleThreaded {
    pub render_mod: fn() -> Colored,
}

impl SingleThreaded {
    pub fn new(render_mod: fn() -> Colored) -> Self {
        Self { render_mod }
    }
}

pub struct Background {
    background_thread: JoinHandle<()>,
    rx: mpsc::Receiver<Colored>,
}

impl Background {
    pub fn new(background_renderer: fn(rx: mpsc::Sender<Colored>)) -> Self {
        let (tx, rx) = mpsc::channel::<Colored>();
        Background {
            background_thread: thread::spawn(move || background_renderer(tx)),
            rx,
        }
    }
}

pub struct MultiThreaded {
    render: Arc<fn(prev: Vec<Colored>) -> Colored>,
    prev_data: Arc<Mutex<Vec<Colored>>>,
}

impl MultiThreaded {
    pub fn new(render: fn(prev: Vec<Colored>) -> Colored) -> Self {
        MultiThreaded {
            render: Arc::new(render),
            prev_data: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn start_render_thread(&self) -> JoinHandle<Colored> {
        let render = self.render.clone();
        let prev = self.prev_data.clone();
        // thread::spawn(move || (*render.as_ref())((**prev).to_vec()))
        thread::spawn(move || {
            let mut prevlist = prev.lock().unwrap();
            let r = render(prevlist.clone());

            prevlist.push(r.clone());
            r
        })
    }
}
