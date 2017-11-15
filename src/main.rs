extern crate clap;

mod external_commands;
mod fifo_reader;

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

    fifo_reader::read_loop(std::path::Path::new(fifo_path));
}

