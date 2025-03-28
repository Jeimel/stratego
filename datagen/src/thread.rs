use std::sync::atomic::AtomicBool;

/// Data obtained from search, which stores the following
/// 1. current position to obtain network input during training
/// 2. moves with visit count to obtain probabilities
/// 3. game outcome either -1, 0 or 1
pub struct SearchData {
    pos: String,
}

pub struct DatagenThread {}

impl DatagenThread {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, iterations: usize, abort: &AtomicBool) {}
}
