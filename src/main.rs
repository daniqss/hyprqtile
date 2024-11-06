mod minimize;
mod prelude;
mod workspace;
use prelude::*;

use crate::minimize::MinimizeCommand;
use crate::workspace::WorkspaceCommand;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = HyprQtileArgs::parse_args();

    match args.command {
        HyprQtileCommand::Workspace(workspace) => workspace.command(),
        HyprQtileCommand::Minimize(minimaze) => minimaze.command(),
    }
}

#[derive(Parser, Debug)]
#[command(
    about = "Qtile-like workspaces and monitors management for the Hyprland compositor",
    arg_required_else_help = true
)]
pub struct HyprQtileArgs {
    #[command(subcommand)]
    pub command: HyprQtileCommand,
}

#[derive(Subcommand, Debug)]
pub enum HyprQtileCommand {
    Workspace(WorkspaceCommand),
    Minimize(MinimizeCommand),
}

impl HyprQtileArgs {
    pub fn parse_args() -> Self {
        HyprQtileArgs::parse()
    }
}
