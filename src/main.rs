mod error;
mod maximize;
mod minimize;
mod prelude;
mod toggle;
mod workspace;
use hyprqtile::Result;

use crate::maximize::MaximizeCommand;
use crate::minimize::MinimizeCommand;
use crate::toggle::ToggleCommand;
use crate::workspace::WorkspaceCommand;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = HyprQtileArgs::parse_args();

    match args.command {
        HyprQtileCommand::Workspace(workspace) => workspace.command(),
        HyprQtileCommand::Minimize(minimaze) => minimaze.command(),
        HyprQtileCommand::Maximize(maximize) => maximize.command(),
        HyprQtileCommand::Toggle(toggle) => toggle.command(),
    }
}

#[derive(Parser, Debug)]
#[command(
    about = "Qtile-like workspaces and monitors management for the Hyprland compositor",
    arg_required_else_help = true,
    version
)]
pub struct HyprQtileArgs {
    #[command(subcommand)]
    pub command: HyprQtileCommand,
}

#[derive(Subcommand, Debug)]
pub enum HyprQtileCommand {
    Workspace(WorkspaceCommand),
    Toggle(ToggleCommand),
    Minimize(MinimizeCommand),
    Maximize(MaximizeCommand),
}

impl HyprQtileArgs {
    pub fn parse_args() -> Self {
        HyprQtileArgs::parse()
    }
}
