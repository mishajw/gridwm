extern crate regex;

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

