use external_commands::ExternalCommand;
use std::error::Error;

pub trait BaseWmError : Error {}

pub trait BaseWm {
    fn handle(&self, command: &ExternalCommand) -> Result<(), &BaseWmError>;
}

