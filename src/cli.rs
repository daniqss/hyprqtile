use clap::{Args, Parser, Subcommand};

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
    /// Moves to the specified workspace
    Workspace(WorkspaceCommand),
    /// Minimizes the specified window
    Minimize(MinimizeCommand),
}

#[derive(Args, Debug)]
pub struct WorkspaceCommand {
    /// The workspace to move to
    pub workspace: Option<i32>,
    /// Moves to the previous workspace
    #[arg(short, long)]
    pub previous: bool,
    /// Moves to the next workspace
    #[arg(short, long)]
    pub next: bool,
}

#[derive(Args, Debug)]
pub struct MinimizeCommand {
    /// The window identifier to minimize
    pub window_identifier: Option<String>,
    /// Minimizes the active window
    #[arg(short, long)]
    pub active: bool,
}

impl HyprQtileArgs {
    pub fn parse_args() -> Self {
        HyprQtileArgs::parse()
    }
}
