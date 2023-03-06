use std::ops::{Sub, Add};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{System, SystemExt, CpuExt};
use chrono::{DateTime, Utc, SecondsFormat};
use log;
use crate::pwdata::PWSample;
use crate::pwreader::PWReader;
extern crate timer;
extern crate chrono;
extern crate page_size;
extern crate sysconf;

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
        let proc_stat_values;
        let mut _proc_stat_content;
        match proc_stat_content {
            Err(e) => {
                log::warn!("Failed to process process stat: {}", e);
                return None
            },
            Ok(content) => {
                _proc_stat_content = content;
                proc_stat_values = _proc_stat_content.split(" ");
            }
        };
        let proc_stat_values: Vec<&str> = proc_stat_values.collect();

        let proc_uptime = match proc_stat_values.get(13)
            .unwrap_or(&"0")
            .parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Failed to parse proc stats: {}", e);
                    return None
                }
        };
        let proc_stime = match proc_stat_values.get(14)
            .unwrap_or(&"0")
            .parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Failed to parse proc stats");
                    return None
                }
        };
        let proc_start_time = match proc_stat_values.get(21)
            .unwrap_or(&"0")
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

        let mut cpu_time: u64 = 0;
        for val_str in stat_values {
            cpu_time += val_str.parse::<u64>().unwrap_or(0u64);
        }

        if current_state.last_cpu_time <= 0 || current_state.last_proc_cpu_time <= 0 {
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

        // RSS
        let page_size = page_size::get();
        let rss: u64 = proc_stat_values.get(23)
            .unwrap_or(&"0")
            .parse::<u64>()
            .unwrap_or(0u64)*(page_size as u64);

        // Total mem
        let total_mem = Self::read_total_mem();

        // Num threads
        let num_threads  = proc_stat_values
            .get(19)
            .unwrap_or(&"0")
            .parse::<i64>()
            .unwrap_or(0);

        // Niceness
        let niceness  = proc_stat_values
            .get(18)
            .unwrap_or(&"0")
            .parse::<i64>()
            .unwrap_or(0);

        // State
        let state = proc_stat_values
            .get(2)
            .unwrap_or(&"");

        // Virtual size
        let vsize = proc_stat_values
            .get(22)
            .unwrap_or(&"0")
            .parse::<i64>()
            .unwrap_or(0i64);

        // Start
        let clock_tick = match sysconf::raw::sysconf(sysconf::raw::SysconfVariable::ScClkTck) {
            Err(e) => {
                log::warn!("Could not retrieve clock tick: {:?}", e);
                0i64
            },
            Ok(v) => v as i64
        };
        let start_time_ms = ((proc_stat_values
            .get(21)
            .unwrap_or(&"0")
            .parse::<i64>()
            .unwrap_or(0i64) as f64)/(clock_tick as f64)*1000f64).round() as u64;

        let uptime_ms = PWSampler::read_sys_uptime_millis();
        let proc_uptime_ms = match uptime_ms {
            None => 0,
            Some(v) => v - start_time_ms
        };
        log::info!("uptime: {}", proc_uptime_ms);
        let start_time_proc: DateTime<Utc> = SystemTime::now()
            .sub(Duration::from_millis(proc_uptime_ms))
            .into();

        sample.ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0)).as_millis() as i64;
        sample.cpu = cpu;
        sample.num_threads = num_threads;
        sample.nice = niceness;
        sample.state = String::from(*state);
        sample.vm_size = vsize;
        sample.rss_size = rss as i64;
        sample.uptime = proc_uptime_ms as i64;
        sample.ram_size = match total_mem {
            None => 0,
            Some(v) => v as i64
        };
        sample.start_time = start_time_proc.to_rfc3339_opts(SecondsFormat::Secs, false);
        sample.ram_size = match total_mem {
            None => 0,
            Some(v) => v as i64
        };

        Some(sample)
    }

    fn read_total_mem() -> Option<u64> {
        lazy_static! {
            static ref RE: Regex = Regex::new("MemTotal:\\s+(\\d+)\\s+kB").unwrap();
        }

        let meminfo_content = match PWReader::read_meminfo() {
            Err(_) => return None,
            Ok(content) => content
        };

        for cap in RE.captures_iter(&meminfo_content) {
            return Some(cap[1].parse::<u64>().unwrap_or(0u64)*1024u64);
        }

        None
    }

    fn read_sys_uptime_millis() -> Option<u64> {
        let content = match PWReader::read_uptime() {
            Err(e) => return None,
            Ok(v) => v
        };

        let tokens: Vec<&str> = content.split(" ").collect();
        if tokens.len() != 2 {
            log::warn!("Cannot parse /proc/uptime content");
            return None
        }

        return Some((tokens
            .get(0)
            .unwrap_or(&"0")
            .parse::<f64>()
            .unwrap_or(0f64)*1000f64).round() as u64);
    }
}
