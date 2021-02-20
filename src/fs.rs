use std::cell::RefCell;
use std::fs::{ReadDir, DirEntry};
use std::path::{PathBuf};
use std::io;

pub struct Scan {
    pub shtml_files: Vec<PathBuf>,
    pub other_files: Vec<PathBuf>
}

pub fn scan(path: &PathBuf, follow_links: bool) -> io::Result<Scan> {

    let mut scan_result = Scan {
        shtml_files: Vec::new(),
        other_files: Vec::new()
    };

    for entry in walkdir::WalkDir::new(path).follow_links(follow_links) {
        let entry = entry?;
        let file_path = entry.path().to_owned();

        let extension = file_path.extension();

        if extension.map_or(false, |e| e == "shtml")
            || extension.map_or(false) {
            scan_result.shtml_files.push(file_path);
        }
        else {
            scan_result.other_files.push(file_path);
        }
    }

    return Ok(scan_result);
}