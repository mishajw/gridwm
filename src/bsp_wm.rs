use error_chain::ChainedError;
use std::io::Read;
use std::process::{Command, Stdio};
use std::str::from_utf8;

use error::*;
use named_workspace_wm::NamedWorkspaceWm;
use workspace::{Workspace, Mode};
use workspace_vector::{WorkspaceVector};

pub struct BspWm {}

impl BspWm {
    pub fn new() -> BspWm {
        BspWm {}
    }

    fn parse_bspc_workspace_str(s: &str) -> Result<Workspace> {
        let workspace_flag: char = s.chars().next()
            .chain_err(|| "String is too short")?;

        let is_focused = !workspace_flag.is_lowercase();
        let mode = match workspace_flag.to_lowercase().next() {
            Some('o') => Mode::Occupied,
            Some('f') => Mode::Unoccupied,
            Some('u') => Mode::Urgent,
            _ => return Err(
                ErrorKind::LogicError(
                    "Unrecognized workspace flag from bspc".to_string()).into())
        };

        let workspace_name = &s[1..];
        WorkspaceVector::from_str(workspace_name)
            .chain_err(|| "Couldn't parse workspace name to vector")
            .map(|wv| Workspace::new(wv, is_focused, mode))
    }

    fn call_bspc(&self, arguments: Vec<&str>) -> Result<()> {
        let status = Command::new("bspc")
            .args(arguments)
            .status()
            .chain_err(|| "Couldn't get result of call")?;

        if status.success() {
            Ok(())
        } else {
            Err(ErrorKind::LogicError("BSPC call returned error code".into()).into())
        }
    }
}

impl NamedWorkspaceWm for BspWm {
    fn get_workspaces(&self) -> Result<Vec<Workspace>> {
        // Start the command
        let command = Command::new("bspc").arg("subscribe")
            .stdout(Stdio::piped())
            .spawn()
            .chain_err(|| "Couldn't run bspc subscribe")?;

        // Read a line from the command
        let mut buffer = [0; 512];
        let read_size: usize = command.stdout
            .chain_err(|| "Couldn't get stdout from process")
            .and_then(|mut stdout| stdout.read(&mut buffer)
                    .chain_err(|| "Couldn't read from process"))?;
        let workspaces_str: &str = from_utf8(&buffer[..read_size])
            .chain_err(|| "Couldn't get string from UTF-8".to_string())?;

        let workspace_pieces: Vec<&str> = workspaces_str.split(":").collect();
        let mut current_monitor_name: Option<&str> = None;
        let mut workspaces: Vec<Workspace> = Vec::new();

        for piece in workspace_pieces {
            if piece.starts_with("WM") {
                current_monitor_name = Some(&piece[2..]);
                continue
            } else if piece.starts_with("L") || piece.starts_with("T") || piece.starts_with("G") {
                // Layout, tiled mode, or flag of the monitor, we don't care
                continue
            } else {
                // Otherwise, must be a workspace
                match BspWm::parse_bspc_workspace_str(piece) {
                    Ok(workspace) => workspaces.push(workspace),
                    Err(e) => println!("{}", e.display_chain().to_string()),
                };
            }

        }

        return Ok(workspaces)
    }

    fn go_to_position(&self, position: &WorkspaceVector) -> Result<()> {
        self.guarentee_exists(position)?;
        self.call_bspc(vec!["desktop" ,"--focus", &position.to_str()])
    }

    fn swap_workspaces(
            &self, position1: &WorkspaceVector, position2: &WorkspaceVector) -> Result<()> {
        self.guarentee_exists(position1)?;
        self.guarentee_exists(position2)?;

        let name1: &str = &position1.to_str();
        let name2: &str = &position2.to_str();
        let name1_tmp: &str = &format!("{}_tmp", &name1);

        self.call_bspc(vec!["desktop", name1, "--rename", name1_tmp])?;
        self.call_bspc(vec!["desktop", name2, "--rename", name1])?;
        self.call_bspc(vec!["desktop", name1_tmp, "--rename", name2])?;
        Ok(())
    }

    fn move_focused_window(&self, new_position: &WorkspaceVector) -> Result<()> {
        let new_position_str: &str = &new_position.to_str();

        self.guarentee_exists(new_position)?;
        self.call_bspc(vec!["node", "--to-desktop", new_position_str])?;
        self.call_bspc(vec!["desktop", "--focus", new_position_str])
    }

    fn guarentee_exists(&self, position: &WorkspaceVector) -> Result<()> {
        let workspaces = self.get_workspaces()?;
        let matching_workspaces: Vec<Workspace> = workspaces
            .into_iter()
            .filter(|w| w.position == *position)
            .collect();

        if matching_workspaces.len() != 1 {
            self.call_bspc(vec!["monitor", "--add-desktops", &position.to_str()])?;
        }

        Ok(())
    }
}

