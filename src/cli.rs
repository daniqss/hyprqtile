use clap::Parser;

/// Qtile-like workspaces and monitors management for the
/// Hyprland compositor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Workspace to move into
    #[arg(short, long)]
    workspace_id: i32,
}

pub fn parse() -> i32 {
    Args::parse().workspace_id
}
