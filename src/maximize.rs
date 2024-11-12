use crate::prelude::*;
use clap::Args;
use hyprqtile::{get_current_workspace, get_workspaces_windows_addresses, move_window, Workspace};

/// Maximizes the specified window
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct MaximizeCommand {
    /// Maximizes the active window
    #[arg(short, long, conflicts_with = "all")]
    pub active: bool,
    /// Maximizes all windows in the active workspace
    #[arg(long, conflicts_with = "active")]
    pub all: bool,
}

impl MaximizeCommand {
    pub fn command(self) -> Result<()> {
        match self {
            Self { active: true, .. } => self.active(),
            Self { all: true, .. } => self.all(),
            _ => Ok(()),
        }
    }

    fn active(self) -> Result<()> {
        let workspace = get_current_workspace()?;
        move_window(Workspace::Id(workspace), None)
    }

    fn all(self) -> Result<()> {
        let workspace = get_current_workspace()?;
        get_workspaces_windows_addresses()?
            .iter()
            .try_for_each(|window| move_window(Workspace::Id(workspace), Some(window)))
    }
}
