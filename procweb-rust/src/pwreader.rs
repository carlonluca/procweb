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

use std::fs;
use std::path::{Path, PathBuf};
use std::io::Error;

///
/// Reads data from the proc filesystem.
/// 
pub struct PWReader {}

impl PWReader {
    pub fn proc_dir(pid: i64) -> PathBuf {
        Path::new("/proc").join(pid.to_string())
    }

    pub fn proc_stat_dir(pid: i64) -> PathBuf {
        Path::new(&Self::proc_dir(pid)).join("stat")
    }

    pub fn proc_status_dir(pid: i64) -> PathBuf {
        Path::new(&Self::proc_dir(pid)).join("status")
    }

    pub fn proc_io_dir(pid: i64) -> PathBuf {
        Path::new(&Self::proc_dir(pid)).join("io")
    }

    pub fn read_cmd_line(pid: i64) -> Result<String, Error> {
        Self::read_file(Path::new(&Self::proc_dir(pid)).join("cmdline"),
            "cmd_line")
    }

    pub fn read_proc_stat(pid: i64) -> Result<String, Error> {
        Self::read_file(Path::new(&Self::proc_stat_dir(pid)).to_path_buf(),
            "stat")
    }

    pub fn read_stat() -> Result<String, Error> {
        Self::read_file(Path::new("/proc/stat").to_path_buf(),
            "proc stat")
    }

    pub fn read_proc_status(pid: i64) -> Result<String, Error> {
        Self::read_file(Self::proc_status_dir(pid),
            "proc status")
    }

    pub fn read_meminfo() -> Result<String, Error> {
        Self::read_file(Path::new("/proc/meminfo").to_path_buf(),
            "meminfo")
    }

    pub fn read_uptime() -> Result<String, Error> {
        Self::read_file(Path::new("/proc/uptime").to_path_buf(),
            "uptime")
    }

    pub fn read_proc_io(pid: i64) -> Result<String, Error> {
        Self::read_file(Self::proc_io_dir(pid), "io")
    }

    fn read_file(file_path: PathBuf, name: &str) -> Result<String, Error> {
        let file_content = fs::read_to_string(file_path);
        match file_content {
            Ok(content) => Ok(content),
            Err(e) => {
                log::warn!("Failed to read {}: {}", name, e);
                Err(e)
            }
        }
    }
}
