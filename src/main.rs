mod external_commands;
mod fifo_reader;

fn main() {
    fifo_reader::read_loop("/tmp/gridwm_fifo");
}

