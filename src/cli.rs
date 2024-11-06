use crate::minimize::MinimizeCommand;
use crate::workspace::WorkspaceCommand;
use clap::{Parser, Subcommand};

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
    /// Moves to the specified workspace, with Qtile monitor behavior
    #[command(arg_required_else_help = true)]
    Workspace(WorkspaceCommand),
    /// Minimizes the specified window
    #[command(arg_required_else_help = true)]
    Minimize(MinimizeCommand),
}

impl HyprQtileArgs {
    pub fn parse_args() -> Self {
        HyprQtileArgs::parse()
    }
}
