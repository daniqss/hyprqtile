mod cli;
mod minimize;
mod prelude;
mod workspace;
use cli::{HyprQtileArgs, HyprQtileCommand};
use prelude::*;

fn main() -> Result<()> {
    let args = HyprQtileArgs::parse_args();

    match args.command {
        HyprQtileCommand::Workspace(workspace) => workspace.command(),
        HyprQtileCommand::Minimize(minimaze) => minimaze.command(),
    }
}
