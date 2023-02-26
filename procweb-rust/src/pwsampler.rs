use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::iter::Enumerate;
use sysinfo::{System, SystemExt, CpuExt};
use log;
use crate::pwdata::PWSample;
use crate::pwreader::PWReader;
extern crate timer;
extern crate chrono;

pub struct PWSamplerData {
    pub last_cpu_time: u64,
    pub last_proc_cpu_time: u64
}

pub struct PWSampler {
    pid: i64,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    samples: Arc<Mutex<Vec<PWSample>>>
}

impl PWSampler {
    pub fn new(pid: i64) -> PWSampler {
        log::info!("Sampler started");
        PWSampler {
            pid: pid,
            thread_handle: None,
            samples: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn start(&mut self) {
        let samples = self.samples.clone();
        let pid = self.pid;
        self.thread_handle = Some(thread::spawn(move || {
            let mut state = PWSamplerData {
                last_cpu_time: 0,
                last_proc_cpu_time: 0
            };
            loop {
                {
                    let mut data = samples.lock().unwrap();
                    match PWSampler::acquire_sample(pid, &mut state) {
                        Some(sample) => {
                            log::info!("Sample: {:?}", sample);
                            (*data).push(sample)
                        },
                        None => {}
                    };
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
    fn acquire_sample_systemcrate(sys: &mut System) -> PWSample {
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

    fn acquire_sample(pid: i64, current_state: &mut PWSamplerData) -> Option<PWSample> {
        let mut sample = PWSample::default();
        let proc_stat_content = PWReader::read_proc_stat(pid);
        let mut proc_stat_lines;
        let mut _proc_stat_content;
        match proc_stat_content {
            Err(e) => return None,
            Ok(content) => {
                _proc_stat_content = content;
                proc_stat_lines = _proc_stat_content.split(" ");
            }
        };

        let proc_uptime = match proc_stat_lines.nth(13)
            .unwrap_or("0")
            .parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Failed to parse proc stats");
                    return None
                }
        };
        let proc_stime = match proc_stat_lines.nth(14)
            .unwrap_or("0")
            .parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Failed to parse proc stats");
                    return None
                }
        };
        let proc_start_time = match proc_stat_lines.nth(21)
            .unwrap_or("0")
            .parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Failed to parse proc stats: {}", e);
                    return None
                }
        };

        let proc_usage_ticks = proc_uptime + proc_stime;
        let stat_content = match PWReader::read_stat() {
            Ok(content) => content,
            Err(e) => {
                log::warn!("Failed to read stat file: {}", e);
                return None;
            }
        };
        let stat_content = stat_content.trim();
        let mut stat_lines = stat_content
            .split("\n")
            .filter(|&x| !x.is_empty());
        let stat_values: Vec<&str> = stat_lines.nth(0)
            .unwrap_or("")
            .split(" ")
            .filter(|&x| !x.is_empty())
            .collect();
        //let stat_values_iterator = stat_values.enumerate();
        //if stat_values_iterator.count() <= 0 {
        //    log::warn!("Failed to parse cpu stats file");
        //    return None;
        //}

        let mut cpu_time: u64 = 0;
        for val_str in stat_values {
            cpu_time += val_str.parse::<u64>().unwrap_or(0u64);
        }

        if current_state.last_cpu_time < 0 || current_state.last_proc_cpu_time < 0 {
            current_state.last_cpu_time = cpu_time;
            current_state.last_proc_cpu_time = proc_usage_ticks;
            return None
        }
        
        let cpu: f64 = if cpu_time - current_state.last_cpu_time == 0 {
            0f64
        }
        else {
            (proc_usage_ticks - current_state.last_proc_cpu_time) as f64/
                (cpu_time - current_state.last_cpu_time) as f64
        };

        sample.cpu = cpu;

        Some(sample)
    }
}
