mod cli;
mod ipc;
mod prelude;
use cli::HyprQtileArgs;
use ipc::*;
use prelude::*;

fn main() -> Result<()> {
    let args = HyprQtileArgs::parse_args();

    match args {
        HyprQtileArgs {
            workspace: Some(workspace),
            ..
        } => move_to(workspace)?,
        HyprQtileArgs { previous: true, .. } => move_to_previous()?,
        HyprQtileArgs { next: true, .. } => move_to_next()?,
        _ => (),
    }

    Ok(())
}
