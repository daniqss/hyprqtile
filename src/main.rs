mod cli;
mod ipc;
mod prelude;
use cli::HyprQtileArgs;
use ipc::*;
use prelude::*;

fn main() -> Result<()> {
    let args = HyprQtileArgs::parse_args();

    println!("{:?}", args);

    match args {
        HyprQtileArgs {
            workspaces: Some(ws),
            ..
        } => move_to(ws)?,
        HyprQtileArgs { previous: true, .. } => move_to_previous()?,
        HyprQtileArgs { next: true, .. } => move_to_next()?,
        _ => (),
    }

    Ok(())
}
