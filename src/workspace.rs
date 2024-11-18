use clap::Args;
use hyprqtile::Result;
use hyprqtile::{get_current_workspace, move_workspace_to};

/// Moves to the specified workspace, with Qtile monitor behavior
#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct WorkspaceCommand {
    /// The workspace to move to
    pub workspace: Option<i32>,
    /// Moves to the previous workspace
    #[arg(short, long, conflicts_with("workspace"), conflicts_with("next"))]
    pub previous: bool,
    /// Moves to the next workspace
    #[arg(short, long, conflicts_with("workspace"), conflicts_with("previous"))]
    pub next: bool,
}

impl WorkspaceCommand {
    pub fn command(self) -> Result<()> {
        match self {
            #[rustfmt::skip]
            Self { workspace: Some(workspace), .. } => move_workspace_to(workspace),
            Self { previous: true, .. } => self.move_to_previous(),
            Self { next: true, .. } => self.move_to_next(),
            _ => Ok(()),
        }
    }

    fn move_to_previous(self) -> Result<()> {
        match get_current_workspace() {
            Ok(workspace) => move_workspace_to(Into::<i32>::into(workspace) - 1),
            Err(hypr_error) => Err(hypr_error),
        }
    }

    fn move_to_next(self) -> Result<()> {
        match get_current_workspace() {
            Ok(workspace) => move_workspace_to(Into::<i32>::into(workspace) + 1),
            Err(hypr_error) => Err(hypr_error),
        }
    }
}
