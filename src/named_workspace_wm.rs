use base_wm::BaseWm;
use error::*;
use external_commands::ExternalCommand;
use workspace::Workspace;
use workspace_vector::WorkspaceVector;

pub trait NamedWorkspaceWm {
    fn get_workspaces(&self) -> Result<Vec<Workspace>>;
    fn go_to_position(&self, position: &WorkspaceVector) -> Result<()>;
    fn swap_workspaces(
            &self, position1: &WorkspaceVector, position2: &WorkspaceVector) -> Result<()>;
    fn move_focused_window(&self, new_position: &WorkspaceVector) -> Result<()>;
    fn guarentee_exists(&self, position: &WorkspaceVector) -> Result<()>;

    fn get_focused_window(&self) -> Result<Workspace> {
        for workspace in self.get_workspaces()? {
            if workspace.is_focused {
                return Ok(workspace)
            }
        }

        Err(ErrorKind::LogicError("No window flagged as focused".into()).into())
    }

    fn handle_direction_command(&self, command: &ExternalCommand) -> Result<()> {
        let direction = match command {
            &ExternalCommand::Go(ref direction) => direction,
            &ExternalCommand::MoveWorkspace(ref direction) => direction,
            &ExternalCommand::MoveWindow(ref direction) => direction,
        };

        let focused_position = self.get_focused_window().map(|w| w.position)?;
        let new_position = &focused_position + &direction.to_vector();
        self.guarentee_exists(&new_position)?;

        match command {
            &ExternalCommand::Go(_) =>
                self.go_to_position(&new_position),
            &ExternalCommand::MoveWorkspace(_) =>
                self.swap_workspaces(&focused_position, &new_position),
            &ExternalCommand::MoveWindow(_) =>
                self.move_focused_window(&new_position),
        }
    }
}

impl<T> BaseWm for T where T: NamedWorkspaceWm {
    fn handle(&self, command: &ExternalCommand) -> Result<()> {
        match command {
            &ExternalCommand::Go(_) |
                &ExternalCommand::MoveWorkspace(_) |
                &ExternalCommand::MoveWindow(_) => self.handle_direction_command(command)
        }
    }
}

