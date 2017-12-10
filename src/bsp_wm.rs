use error_chain::ChainedError;
use std::io::BufRead;
use std::io::{Read, BufReader};
use std::ops::Deref;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;


use error::*;
use named_workspace_wm::NamedWorkspaceWm;
use workspace::{Workspace, Mode};
use workspace_vector::{WorkspaceVector};

type ThreadHandle = Option<thread::JoinHandle<()>>;

pub struct BspWm {
    bspc_subscribe_thread: ThreadHandle,
    workspaces: Arc<Mutex<Vec<Workspace>>>,
}

impl BspWm {
    pub fn new() -> BspWm {
        let mut bsp_wm = BspWm {
            bspc_subscribe_thread: None,
            workspaces: Arc::new(Mutex::new(vec![])),
        };

        let cloned_workspaces = bsp_wm.workspaces.clone();
        bsp_wm.bspc_subscribe_thread = Some(thread::spawn(
                || BspWm::start_bspc_subscribe_handler(cloned_workspaces)));

        bsp_wm
    }

    fn start_bspc_subscribe_handler(workspaces: Arc<Mutex<Vec<Workspace>>>) {
        match BspWm::bspc_subscribe_handler(workspaces) {
            Ok(_) => println!("BSPC subscribe thread exitted with no error"),
            Err(e) => println!(
                "BSPC subscribe thread exitted with error:\n{}",
                e.display_chain().to_string()),
        }
    }

    fn bspc_subscribe_handler(workspaces: Arc<Mutex<Vec<Workspace>>>) -> Result<()> {
        // Start the command
        let command = Command::new("bspc").arg("subscribe")
            .stdout(Stdio::piped())
            .spawn()
            .chain_err(|| "Couldn't run bspc subscribe")?;

        // Read a line from the command
        let reader = BufReader::new(
            command.stdout.chain_err(|| "Couldn't get stdout from process")?);

        for line in reader.lines().filter_map(|r| r.ok()) {
            let workspace_pieces: Vec<&str> = line.split(":").collect();
            let mut current_monitor_name: Option<&str> = None;
            let mut new_workspaces: Vec<Workspace> = Vec::new();

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
                        Ok(Some(workspace)) => new_workspaces.push(workspace),
                        Ok(None) => (),
                        Err(e) => println!("{}", e.display_chain().to_string()),
                    };
                }
            }

            let mut workspaces_locked = workspaces.lock().unwrap();
            workspaces_locked.clear();
            workspaces_locked.append(&mut new_workspaces);
        }

        return Ok(())
    }

    fn parse_bspc_workspace_str(s: &str) -> Result<Option<Workspace>> {
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

        match WorkspaceVector::from_str(workspace_name)
                .chain_err(|| "Couldn't parse workspace name to vector") {
            Ok(Some(wv)) => Ok(Some(Workspace::new(wv, is_focused, mode))),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
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
        match self.workspaces.lock() {
            Ok(ws) => Ok(ws.clone()),
            Err(e) => Err(ErrorKind::RuntimeError(
                    "Couldn't get lock on workspaces".into()).into()),
        }
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

