use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    about = "Qtile-like workspaces and monitors management for the Hyprland compositor",
    arg_required_else_help = true
)]
pub struct HyprQtileArgs {
    /// Moves to the specified workspace
    #[arg(short, long, conflicts_with = "next", conflicts_with = "previous")]
    pub workspaces: Option<i32>,

    /// Moves to the previous workspace
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "workspaces", conflicts_with = "next")]
    pub previous: bool,

    /// Moves to the next workspace
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "workspaces", conflicts_with = "previous")]
    pub next: bool,
}

impl HyprQtileArgs {
    pub fn parse_args() -> Self {
        HyprQtileArgs::parse()
    }
}
