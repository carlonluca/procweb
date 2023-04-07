/**
 * Copyright (C) 2023 Luca Carlon. All rights reserved.
 * 
 * This file is part of procweb-rust.
 * 
 * procweb-rust is free software: you can redistribute it and/or modify it under the terms of the GNU
 * General Public License as published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 * 
 * procweb-rust is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 * PURPOSE. See the GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along with procweb-rust. If not,
 * see <https://www.gnu.org/licenses/>.
 */

use std::thread::{self, JoinHandle};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::pwsampler::PWSampler;

pub struct PWSamplerThread<T: 'static, ST: 'static> {
    thread_handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    sampler: Arc<Mutex<dyn PWSampler<T, ST>>>
}

impl<T, ST> PWSamplerThread<T, ST> {
    ///
    /// Creates a new instance.
    /// 
    pub fn new(sampler: Arc<Mutex<dyn PWSampler<T, ST>>>) -> PWSamplerThread<T, ST> {
        PWSamplerThread::<T, ST> {
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
