use std::thread::{self, JoinHandle};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use log::info;

use crate::pwsampler::PWSampler;

pub struct PWSamplerThread<T: 'static> {
    thread_handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    pub sampler: Arc<Mutex<dyn PWSampler<T>>>
}

impl<T> PWSamplerThread<T> {
    ///
    /// Creates a new instance.
    /// 
    pub fn new(sampler: Arc<Mutex<dyn PWSampler<T>>>) -> PWSamplerThread<T> {
        PWSamplerThread::<T> {
            thread_handle: None,
            running: Arc::new(AtomicBool::new(true)),
            sampler: sampler
        }
    }

    ///
    /// Starts the sampler.
    /// 
    pub fn start(&mut self) {
        if self.thread_handle.is_none() {
            let sampler = self.sampler.clone();
            let running = Arc::new(AtomicBool::new(true));
            let thread_handle = thread::spawn({
                let running = running.clone();
                move || {
                    while running.load(Ordering::Relaxed) {
                        {
                            let mut sampler = sampler.lock().unwrap();
                            let sample = sampler.sample();
                        }
                        
                        log::info!("Sample taken");
                        sleep(Duration::from_secs(1));
                    }
                }
            });
            self.thread_handle = Some(thread_handle);
            self.running = running;
        }
    }

    ///
    /// Stops the sampler.
    /// 
    pub fn stop(&mut self) {
        if let Some(thread_handle) = self.thread_handle.take() {
            self.running.store(false, Ordering::Relaxed);
            thread_handle.join().unwrap();
        }
    }
}
