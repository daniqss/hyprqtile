use hyprqtile::prelude::*;
use serde_json::{from_slice, Value};
use std::{collections::HashSet, thread::sleep, time::Duration};

pub const MILLIS_TO_WAIT: u64 = 200;

pub fn exec(args: &str) -> Result<Vec<u8>> {
    match std::process::Command::new("hyprctl")
        .args(args.split(' '))
        .output()
    {
        Ok(output) => Ok(output.stdout),
        Err(error) => Err(error.into()),
    }
}

pub fn parse(raw: &Vec<u8>) -> Result<Value> {
    match from_slice(raw) {
        Ok(value) => Ok(value),
        Err(error) => Err(Error::SerdeJson(error)),
    }
}

pub fn create_window() -> Result<String> {
    let Some(old_clients) = parse(&exec("clients -j")?)?.as_array().cloned() else {
        return Err(Error::Generic("No old clients found".to_string()));
    };
    let old_clients: HashSet<String> = old_clients
        .iter()
        .map(|c| c["address"].to_string())
        .collect();

    exec(
        format!(
            "dispatch exec {}",
            // search for TERM, if not set use kitty as it's the default terminal for hyprland
            std::env::var("TERM").unwrap_or_else(|_| "kitty".to_string())
        )
        .as_str(),
    )?;
    sleep(Duration::from_millis(MILLIS_TO_WAIT));

    let Some(new_clients) = parse(&exec("clients -j")?)?.as_array().cloned() else {
        return Err(Error::Generic("No new clients found".to_string()));
    };
    let new_clients: HashSet<String> = new_clients
        .iter()
        .map(|c| c["address"].to_string())
        .collect();

    let differences: Vec<String> = new_clients.difference(&old_clients).cloned().collect();
    if differences.len() != 1 {
        return Err(Error::Generic(format!(
            "No clients found:\n{:?}",
            differences
        )));
    }

    match differences.into_iter().next() {
        Some(address) => Ok(address.chars().filter(|c| *c != '"').collect()),
        None => Err(Error::Generic("No clients found".to_string())),
    }
}

pub fn move_window_to(workspace: String, address: &String) -> Result<()> {
    let message = format!(
        "dispatch movetoworkspacesilent {},address:{}",
        workspace, address
    );
    println!("{}", message);
    match exec(message.as_str()) {
        Ok(output) => {
            println!("Moved window to workspace: {:?}", String::from_utf8(output));
            Ok(())
        }
        Err(_) => Err(Error::Generic(format!(
            "Failed to move window to workspace: {}",
            message
        ))),
    }
}

pub fn kill_window(address: String) -> Result<()> {
    match exec(format!("dispatch closewindow address:{}", address).as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::Generic(format!(
            "Failed to kill window: {}",
            address
        ))),
    }
}
