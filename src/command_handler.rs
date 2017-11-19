extern crate libc;

use std::ffi::CString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use external_commands::ExternalCommand;
use base_wm::BaseWm;

pub fn run(fifo_path: &Path, wm: &BaseWm) {
    read_loop(fifo_path, wm);
}

fn read_loop(fifo_path: &Path, wm: &BaseWm) {
    if !fifo_path.exists() {
        mkfifo(fifo_path);
    }

    let file = File::open(fifo_path).expect("File not found");
    let reader = BufReader::new(&file);

    for line in reader.lines().filter_map(|r| r.ok()) {
        match ExternalCommand::from(&line) {
            Some(command) =>
                wm.handle(&command)
                    .unwrap_or_else(|e| println!("Couldn't handle command due to error: {}", e)),
            None => println!("Unrecognized command {}", line),
        };
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

