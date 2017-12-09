extern crate clap;
#[macro_use]
extern crate error_chain;

use error_chain::ChainedError;
use std::process;

mod base_wm;
mod bsp_wm;
mod command_handler;
mod error;
mod external_commands;
mod workspace;
mod workspace_vector;

fn main() {
    let matches = clap::App::new("gridwm")
        .arg(clap::Arg::with_name("fifo_path")
            .long("fifo-path")
            .short("f")
            .help("The path of the fifo to read commands from")
            .takes_value(true))
        .get_matches();

    let fifo_path = matches
        .value_of("fifo_path")
        .unwrap_or("/tmp/gridwm_fifo");

    let wm = bsp_wm::BspWm::new();
    if let Err(e) = command_handler::run(std::path::Path::new(fifo_path), &wm) {
        println!("{}", e.display_chain().to_string());
        process::exit(1);
    }
}

