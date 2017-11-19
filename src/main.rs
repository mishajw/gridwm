extern crate clap;

mod base_wm;
mod bsp_wm;
mod external_commands;
mod command_handler;

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
    command_handler::run(std::path::Path::new(fifo_path));
}

