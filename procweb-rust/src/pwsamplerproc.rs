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

use std::ops::Sub;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use regex::Regex;
use chrono::{DateTime, Utc, SecondsFormat};
use log;
use crate::pwdata::PWSampleProc;
use crate::pwdata::PWSetupProc;
use crate::pwreader::PWReader;
use crate::pwsampler::PWSampler;
extern crate timer;
extern crate chrono;
extern crate page_size;
extern crate sysconf;

pub struct PWIoValues {
    pub read: u64,
    pub written: u64
}

pub struct PWSamplerData {
    pub last_cpu_time: u64,
    pub last_proc_cpu_time: u64
}

pub struct PWSamplerProc {
    pid: i64,
    current_state: Option<PWSamplerData>,
    samples: Arc<Mutex<Vec<PWSampleProc>>>,
    setup: PWSetupProc
}

impl PWSampler<PWSampleProc, PWSetupProc> for PWSamplerProc {

    fn samples(&self) -> Arc<Mutex<Vec<PWSampleProc>>> {
        self.samples.clone()
    }

    fn setup(&self) -> &PWSetupProc {
        &self.setup
    }

    fn sample(&mut self) -> Option<PWSampleProc> {
        let mut sample = PWSampleProc::default();
        let proc_stat_content = PWReader::read_proc_stat(self.pid);
        let proc_status_content = match PWReader::read_proc_status(self.pid) {
            Err(_) => String::new(),
            Ok(c) => c
        };
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

        let cpu;
        match &self.current_state {
            None => {
                self.current_state = Some(PWSamplerData {
                    last_cpu_time: cpu_time,
                    last_proc_cpu_time: proc_usage_ticks
                });
                return None;
            },
            Some(current_state) => {
                cpu = if cpu_time - current_state.last_cpu_time == 0 {
                    0f64
                }
                else {
                    (proc_usage_ticks - current_state.last_proc_cpu_time) as f64/
                        (cpu_time - current_state.last_cpu_time) as f64
                };
            }
        }

        self.current_state = Some(PWSamplerData {
            last_cpu_time: cpu_time as u64,
            last_proc_cpu_time: proc_usage_ticks
        });

        // RSS
        let page_size = page_size::get();
        let rss: u64 = proc_stat_values.get(23)
            .unwrap_or(&"0")
            .parse::<u64>()
            .unwrap_or(0u64)*(page_size as u64);

        // Total mem
        let total_mem = self.read_total_mem();

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

        // RSS peak
        lazy_static! {
            static ref VM_HWM_REGEX: Regex = Regex::new("VmHWM:\\s+(\\d+)\\s+kB").unwrap();
        }
        let rss_peak = self.read_if_matches(&VM_HWM_REGEX, &proc_status_content.to_string())*1024;

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

        let uptime_ms = self.read_sys_uptime_millis();
        let proc_uptime_ms = match uptime_ms {
            None => 0,
            Some(v) => v - start_time_ms
        };
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
        sample.rss_peak = rss_peak as i64;
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
        match self.read_io_values(self.pid) {
            None => {},
            Some(v) => {
                sample.read_all = v.1.read as i64;
                sample.read_disk = v.0.read as i64;
                sample.write_all = v.1.written as i64;
                sample.write_disk = v.0.written as i64;
            }
        }

        self.samples.as_ref().lock().unwrap().push(sample.clone());

        Some(sample)
    }
}

impl PWSamplerProc {

    pub fn new(pid: i64) -> PWSamplerProc {
        PWSamplerProc {
            pid: pid,
            current_state: None,
            samples: Arc::new(Mutex::new(Vec::<PWSampleProc>::new())),
            setup: PWSetupProc {
                cmdline: match PWReader::read_cmd_line(pid) {
                    Err(_) => "".to_string(),
                    Ok(v) => v
                },
                pid: pid,
                sample_interval: 1000
            }
        }
    }

    fn read_total_mem(&self) -> Option<u64> {
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

    fn read_sys_uptime_millis(&self) -> Option<u64> {
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

    fn read_io_values(&self, pid: i64) -> Option<(PWIoValues, PWIoValues)> {
        let proc_io_content = match PWReader::read_proc_io(pid) {
            Err(_) => return None,
            Ok(v) => v
        };

        lazy_static! {
            static ref REGEX_RBYTES: Regex = Regex::new(r"read_bytes:\s*(\d*)").unwrap();
            static ref REGEX_WBYTES: Regex = Regex::new(r"write_bytes:\s*(\d*)").unwrap();
            static ref REGEX_RCHAR: Regex = Regex::new(r"rchar:\s+(\d+)").unwrap();
            static ref REGEX_WCHAR: Regex = Regex::new(r"wchar:\s+(\d+)").unwrap();
        }

        let disk = PWIoValues {
            read: self.read_if_matches(&REGEX_RBYTES, &proc_io_content),
            written: self.read_if_matches(&REGEX_WBYTES, &proc_io_content)
        };
        let all = PWIoValues {
            read: self.read_if_matches(&REGEX_RCHAR, &proc_io_content),
            written: self.read_if_matches(&REGEX_WCHAR, &proc_io_content)
        };

        Some((disk, all))
    }

    fn read_if_matches(&self, regex: &Regex, pattern: &String) -> u64 {
        match regex.captures(pattern) {
            None => 0u64,
            Some(v) => v.get(1).unwrap().as_str().parse::<u64>().unwrap_or(0u64)
        }
    }
}
