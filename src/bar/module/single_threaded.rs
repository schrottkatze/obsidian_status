use crate::Colored;

pub struct SingleThreaded {
    pub run: fn() -> Colored,
}
impl SingleThreaded {
    pub fn new(run: fn() -> Colored) -> Self {
        Self { run }
    }
}
