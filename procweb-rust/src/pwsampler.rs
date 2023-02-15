use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use sysinfo::{System, SystemExt, CpuExt};
use log;
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
            let mut sys = System::new_all();
            loop {
                {
                    let mut data = samples.lock().unwrap();
                    (*data).push(PWSampler::acquire_sample(&mut sys));
                }
                
                sleep(Duration::from_secs(1));
            }
        }));
    }

    pub fn samples(&self) -> Arc<Mutex<Vec<PWSample>>> {
        self.samples.clone()
    }

    // Private portion
    // ===============
    fn acquire_sample(sys: &mut System) -> PWSample {
        let now = SystemTime::now();
        let ts = match now.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            Err(err) => {
                log::warn!("Could not get time: {:?}", err);
                Duration::from_millis(0)
            }
        };

        sys.refresh_all();

        let mut avg_cpu = 0f64;
        for cpu in sys.cpus() {
            avg_cpu += cpu.cpu_usage() as f64;
        }
        avg_cpu = avg_cpu/sys.cpus().len() as f64;

        let mut def = PWSample::default();
        def.ts = ts.as_millis() as i64;
        def.cpu = avg_cpu;

        log::info!("Sample: {:?}", def);
        def
    }
}