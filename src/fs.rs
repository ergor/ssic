use std::cell::RefCell;
use std::fs::{ReadDir, DirEntry};
use std::path::PathBuf;
use std::io;

pub struct Scan {
    pub shtml_files: Vec<DirEntry>,
    pub other_files: Vec<DirEntry>
}

pub fn scan(path: PathBuf) -> io::Result<Scan> {

    let mut scan_result = Scan {
        shtml_files: Vec::new(),
        other_files: Vec::new()
    };

    let mut dir_iter_stack: Vec<RefCell<ReadDir>> = Vec::new();

    let root_dir_iter = std::fs::read_dir(path)?;
    dir_iter_stack.push(RefCell::new(root_dir_iter));

    while !dir_iter_stack.is_empty() {
        // safety: we know it's Some, because of loop condition
        let current_dir_iter = dir_iter_stack.last().unwrap();

        let next_child_opt = current_dir_iter.borrow_mut().next();
        if let Some(child) = next_child_opt {
            match child {
                Ok(child) => {
                    // if ends with shtml or has x bit set, push to shtml files
                    // else push to other files

                    if child.file_type()
                },
                Err(e) => {}
            }
        }
    }

    return Ok(scan_result);
}