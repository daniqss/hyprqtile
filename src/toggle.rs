use crate::prelude::*;
use clap::Args;
use hyprqtile::toggle_special_workspace;

/// Toggle from the current workspace to its special workspace
#[derive(Args, Debug)]
#[command()]
pub struct ToggleCommand {}
impl ToggleCommand {
    pub fn command(self) -> Result<()> {
        toggle_special_workspace()
    }
}
