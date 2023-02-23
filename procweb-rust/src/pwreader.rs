use std::fs;
use std::path::{Path, PathBuf};

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

    pub fn read_cmd_line(pid: i64) -> Option<String> {
        Self::read_file(Path::new(&Self::proc_dir(pid)).join("cmdline"),
            "cmd_line")
    }

    pub fn read_proc_stat(pid: i64) -> Option<String> {
        Self::read_file(Path::new(&Self::proc_stat_dir(pid)).to_path_buf(),
            "stat")
    }

    fn read_file(file_path: PathBuf, name: &str) -> Option<String> {
        let file_content = fs::read_to_string(file_path);
        match file_content {
            Ok(content) => Some(content),
            Err(e) => {
                log::warn!("Failed to read {}: {}", name, e);
                None
            }
        }
    }
}
