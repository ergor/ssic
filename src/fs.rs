use std::cell::RefCell;
use std::fs::{ReadDir, DirEntry};
use std::path::PathBuf;
use std::io;

pub struct Scan {
    pub shtml_files: Vec<DirEntry>,
    pub other_files: Vec<DirEntry>
}

pub fn scan(path: PathBuf, follow_links: bool) -> io::Result<Scan> {

    let mut scan_result = Scan {
        shtml_files: Vec::new(),
        other_files: Vec::new()
    };

    for entry in walkdir::WalkDir::new(path).follow_links(follow_links) {

    }

    return Ok(scan_result);
}