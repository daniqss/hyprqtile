use crate::{ipc::toggle_special_workspace, prelude::*};
use clap::Args;

/// Toggle from the current workspace to its special workspace
#[derive(Args, Debug)]
#[command()]
pub struct ToggleCommand {}
impl ToggleCommand {
    pub fn command(self) -> Result<()> {
        toggle_special_workspace()
    }
}
