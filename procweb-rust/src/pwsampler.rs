use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use log::{debug, error, log_enabled, info, Level};
use crate::pwdata::PWSample;
extern crate timer;
extern crate chrono;

pub struct PWSampler {
    pid: i64,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    samples: Arc<Mutex<Vec<PWSample>>>
}

impl PWSampler {
    pub fn new() -> PWSampler {
        log::info!("Start");
        PWSampler {
            pid: 0,
            thread_handle: None,
            samples: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn start(&mut self) {
        let samples = self.samples.clone();
        self.thread_handle = Some(thread::spawn(move || {
            // some work here
            loop {
                let mut data = samples.lock().unwrap();
                (*data).push(PWSampler::acquire_sample());
                sleep(Duration::from_secs(1));
                log::info!("Sample: {:?}", (*data).last());
            }
        }));
    }

    // Private portion
    // ===============
    fn acquire_sample() -> PWSample {
        return PWSample::default();
    }
}