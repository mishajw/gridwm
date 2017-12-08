extern crate regex;

use workspace_vector::WorkspaceVector;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn from(s: &str) -> Option<Direction> {
        match s {
            "left" => Some(Direction::Left),
            "right" => Some(Direction::Right),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None
        }
    }

    pub fn to_vector(&self) -> WorkspaceVector {
        match *self {
            Direction::Left => WorkspaceVector::new(-1, 0),
            Direction::Right => WorkspaceVector::new(1, 0),
            Direction::Up => WorkspaceVector::new(0, -1),
            Direction::Down => WorkspaceVector::new(0, 1),
        }
    }
}

#[derive(Debug)]
pub enum ExternalCommand {
    Go(Direction),
    MoveWorkspace(Direction),
    MoveWindow(Direction)
}

impl ExternalCommand {
    pub fn from(s: &str) -> Option<ExternalCommand> {
        if let Some(space_index) = s.find(" ") {
            let first_word = &s[..space_index];
            let rest = &s[space_index + 1..];
            return match first_word {
                "go" => Direction::from(rest).map(ExternalCommand::Go),
                "move-ws" => Direction::from(rest).map(ExternalCommand::MoveWorkspace),
                "move-win" => Direction::from(rest).map(ExternalCommand::MoveWindow),
                _ => None
            }
        } else {
            None
        }
    }
}

