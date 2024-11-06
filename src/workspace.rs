use crate::prelude::*;
use clap::Args;
use hyprqtile::{move_to, move_to_next, move_to_previous};

#[derive(Args, Debug)]
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
            Self {
                workspace: Some(workspace),
                ..
            } => move_to(workspace),
            Self { previous: true, .. } => move_to_previous(),
            Self { next: true, .. } => move_to_next(),
            _ => Ok(()),
        }
    }
}
