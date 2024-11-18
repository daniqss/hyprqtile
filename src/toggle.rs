use clap::Args;
use hyprqtile::toggle_special_workspace;
use hyprqtile::Result;

/// Toggle from the current workspace to its special workspace
#[derive(Args, Debug)]
#[command()]
pub struct ToggleCommand {}
impl ToggleCommand {
    pub fn command(self) -> Result<()> {
        toggle_special_workspace()
    }
}
