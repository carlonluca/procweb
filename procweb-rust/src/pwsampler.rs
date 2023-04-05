use std::sync::{Arc, Mutex};

pub trait PWSampler<S>: Sync + Send {
    fn sample(&mut self) -> Option<S>;
    fn samples(&self) -> Arc<Mutex<Vec<S>>>;
}
