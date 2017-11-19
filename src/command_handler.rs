extern crate libc;

use std::ffi::CString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use external_commands::ExternalCommand;

pub fn run(fifo_path: &Path) {
    read_loop(fifo_path);
}

fn read_loop(fifo_path: &Path) {
    if !fifo_path.exists() {
        mkfifo(fifo_path);
    }

    let file = File::open(fifo_path).expect("File not found");
    let reader = BufReader::new(&file);

    for line in reader.lines().filter_map(|r| r.ok()) {
        let command = ExternalCommand::from(&line);
        
        println!("{:?}", command)
    }
}

fn mkfifo(path: &Path) {
    unsafe {
        // TODO: Use nix::mkfifo once it has been released
        let path_str = path.to_str().expect("Path is not unicode");
        let c_path = CString::new(path_str).unwrap();
        libc::mkfifo(c_path.as_ptr(), 0o666);
    }
}
