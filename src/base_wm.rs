use external_commands::ExternalCommand;
use error;

pub trait BaseWm {
    fn handle(&self, command: &ExternalCommand) -> error::Result<()>;
}

