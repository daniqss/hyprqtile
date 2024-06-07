mod cli;
mod ipc;
mod prelude;
use cli::parse;
use ipc::*;
use prelude::*;

fn main() -> Result<()> {
    let workspace = parse();

    move_to(workspace)?;

    Ok(())
}
