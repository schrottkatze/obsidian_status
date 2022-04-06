pub mod background;
pub mod multi_threaded;
pub mod single_threaded;

use crate::bar::module::background::Background;
use crate::bar::module::multi_threaded::MultiThreaded;
use crate::bar::module::single_threaded::SingleThreaded;
use crate::Colored;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
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
    pub fn new_background(run: fn(Arc<Mutex<Option<Colored>>>, Sender<()>)) -> Module {
        Module::Background(Background::new(run))
    }
}
