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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct PWSampleProc {
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

impl Default for PWSampleProc {
    fn default() -> PWSampleProc {
        PWSampleProc {
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
pub struct PWSetupProc {
    pub cmdline: String,
    pub pid: i64,
    pub sample_interval: i64
}