use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use external_commands::ExternalCommand;

pub fn read_loop(fifo_path: &str) {
    let file = File::open(fifo_path).expect("File not found");
    let reader = BufReader::new(&file);

    for line in reader.lines().filter_map(|r| r.ok()) {
        let command = ExternalCommand::from(&line);
        
        println!("{:?}", command)
    }
}

