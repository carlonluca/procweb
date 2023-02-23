use std::str;
use std::fmt;
use std::fs;
use std::path::Path;

struct PWReader {}

impl PWReader {
    pub fn procDir(pid: i64) -> String {
        format!("/proc/{}", pid)
    }

    pub fn procStatDir(pid: i64) -> String {
        Path::new(procDir(pid)).join("stat").to_str()
    }

    pub fn procIoDir(pid: i64) -> Option<str> {
        Path::new(procDir(pid)).join("io").to_str()
    }

    pub fn readCmdline(pid: i64) {
        let filePath = Path::new(procDir(pid)).join("cmdline");
        let fileContent = fs::read_to_string(filePath);
        match fileContent {
            
        }
    }
}
