use base_wm::{BaseWm, BaseWmError};
use external_commands::ExternalCommand;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum BspWmError {
    ExecError
}

impl BaseWmError for BspWmError {}

impl error::Error for BspWmError {
    fn description(&self) -> &str {
        "Error executring bspc command"
    }
}

impl fmt::Display for BspWmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BspWmError")
    }
}

pub struct BspWm {}

impl BspWm {
    pub fn new() -> BspWm {
        BspWm {}
    }
}

impl BaseWm for BspWm {
    fn handle(&self, command: &ExternalCommand) -> Result<(), &BaseWmError> {
        Err(&BspWmError::ExecError)
    }
}

