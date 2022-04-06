use crate::Colored;
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type BackgroundRunFn = fn(Arc<Mutex<Option<Colored>>>, Sender<()>);

pub struct Background {
    background_thread_handle: RefCell<Option<JoinHandle<()>>>,
    run: Arc<BackgroundRunFn>,
    latest: Arc<Mutex<Option<Colored>>>,
}
impl Background {
    pub fn new(run: BackgroundRunFn) -> Self {
        Background {
            run: Arc::new(run),
            background_thread_handle: RefCell::new(None),
            latest: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self, tx: Sender<()>) {
        if self.is_not_running() {
            let run = self.run.clone();
            let current = self.latest.clone();
            *self.background_thread_handle.borrow_mut() =
                Some(thread::spawn(move || run(current, tx)));
        }
    }

    pub fn is_not_running(&self) -> bool {
        self.background_thread_handle.borrow().is_some()
    }

    pub fn get_latest(&self) -> Colored {
        let latest = self.latest.lock().unwrap();
        match latest.deref() {
            Some(v) => v.clone(),
            None => Colored::from_str(""),
        }
    }
}
