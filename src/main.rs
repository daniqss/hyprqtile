mod cli;
mod ipc;
mod prelude;
use cli::{HyprQtileArgs, HyprQtileCommand};
use ipc::{move_to, move_to_next, move_to_previous};
use prelude::*;

fn main() -> Result<()> {
    let args = HyprQtileArgs::parse_args();

    match args.command {
        HyprQtileCommand::Workspace(workspace) => {
            if workspace.previous {
                move_to_previous()?;
            } else if workspace.next {
                move_to_next()?;
            } else if let Some(workspace) = workspace.workspace {
                move_to(workspace)?;
            }
        }
        HyprQtileCommand::Minimize(minimaze) => {}
    };

    Ok(())
}
