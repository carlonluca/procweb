use std::fs;
use std::path::{Path, PathBuf};
use std::io::Error;

pub struct PWReader {}

impl PWReader {
    pub fn proc_dir(pid: i64) -> PathBuf {
        Path::new("/proc").join(pid.to_string())
    }

    pub fn proc_stat_dir(pid: i64) -> PathBuf {
        Path::new(&Self::proc_dir(pid)).join("stat")
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

    pub fn read_meminfo() -> Result<String, Error> {
        Self::read_file(Path::new("/proc/meminfo").to_path_buf(),
            "meminfo")
    }

    pub fn read_uptime() -> Result<String, Error> {
        Self::read_file(Path::new("/proc/uptime").to_path_buf(),
            "uptime")
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
