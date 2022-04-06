use crate::Colored;
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type MultiThreadedRunFn = fn(prev: Option<Vec<Colored>>) -> Colored;
pub struct MultiThreaded {
    running_thread: RefCell<Option<JoinHandle<Colored>>>,
    run: Arc<MultiThreadedRunFn>,
    prev_data: Arc<Mutex<Option<Vec<Colored>>>>,
}
impl MultiThreaded {
    pub fn new(run: MultiThreadedRunFn, collect_prev_data: bool) -> Self {
        MultiThreaded {
            running_thread: RefCell::new(None),
            run: Arc::new(run),
            prev_data: Arc::new(Mutex::new(if collect_prev_data {
                Some(Vec::<Colored>::new())
            } else {
                None
            })),
        }
    }

    pub fn start(&self) {
        let render = self.run.clone();
        let prev = self.prev_data.clone();
        if self.running_thread.borrow().is_none() {
            self.running_thread.replace(Some(thread::spawn(move || {
                let mut prv = prev.lock().unwrap();
                let r = render(prv.clone());

                if let Some(prv) = prv.deref_mut() {
                    prv.push(r.clone())
                }

                r
            })));
        } else {
            panic!("tried to start a thread while a thread already was running")
        }
    }

    pub fn join_running(&self) -> thread::Result<Colored> {
        if let Some(handle) = self.running_thread.replace(None) {
            handle.join()
        } else {
            panic!("tried to join a running thread without a running thread!")
        }
    }
}
