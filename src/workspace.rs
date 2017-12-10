use workspace_vector::WorkspaceVector;

#[derive(Debug, Clone)]
pub enum Mode {
    Unoccupied,
    Occupied,
    Urgent
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub position: WorkspaceVector,
    pub is_focused: bool,
    pub mode: Mode
}

impl Workspace {
    pub fn new(position: WorkspaceVector, is_focused: bool, mode: Mode) -> Workspace {
        return Workspace {position, is_focused, mode}
    }
}

