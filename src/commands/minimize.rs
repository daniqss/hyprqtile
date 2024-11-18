use crate::{
    ipc::{
        get_active_workspace_windows_addresses, get_current_workspace_id, move_window, Workspace,
    },
    prelude::*,
};
use clap::Args;

/// Minimizes the specified window
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct MinimizeCommand {
    /// Minimizes the active window
    #[arg(short, conflicts_with = "all")]
    pub active: bool,
    /// Minimizes all windows in the active workspace
    #[arg(long, conflicts_with = "active")]
    pub all: bool,
}

impl MinimizeCommand {
    pub fn command(self) -> Result<()> {
        match self {
            Self { active: true, .. } => self.active(),
            Self { all: true, .. } => self.all(),
            _ => Ok(()),
        }
    }

    fn active(self) -> Result<()> {
        let workspace = get_current_workspace_id()?;
        move_window(Workspace::Special(workspace), None)
    }

    fn all(self) -> Result<()> {
        let workspace = get_current_workspace_id()?;
        get_active_workspace_windows_addresses()?
            .iter()
            .try_for_each(|window| move_window(Workspace::Special(workspace), Some(window)))
    }
}
