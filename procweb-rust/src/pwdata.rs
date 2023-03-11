use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSample {
    pub ts: i64,
    pub cpu: f64,
    pub vm_peak: i64,
    pub vm_size: i64,
    pub rss_peak: i64,
    pub rss_size: i64,
    pub ram_size: i64,
    pub num_threads: i64,
    pub nice: i64,
    pub uptime: i64,
    pub read_all: i64,
    pub write_all: i64,
    pub read_disk: i64,
    pub write_disk: i64,
    pub start_time: String,
    pub state: String
}

impl Default for PWSample {
    fn default() -> PWSample {
        PWSample {
            ts: 0,
            cpu: 0f64,
            vm_peak: 0,
            vm_size: 0,
            rss_peak: 0,
            rss_size: 0,
            ram_size: 0,
            num_threads: 0,
            nice: 0,
            uptime: 0,
            read_all: 0,
            write_all: 0,
            read_disk: 0,
            write_disk: 0,
            start_time: String::new(),
            state: String::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSetup {
    pub cmdline: String,
    pub pid: i64,
    pub sample_interval: i64
}