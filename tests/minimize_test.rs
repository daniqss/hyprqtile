use hyprqtile::{commands::MinimizeCommand, prelude::*, workspace};
use std::{thread::sleep, time::Duration};
mod utils;
use utils::*;

#[test]
fn test_minimize_active_command() -> Result<()> {
    // create window that we'll use to get data
    let new_window = create_window()?;
    sleep(Duration::from_millis(MILLIS_TO_WAIT));

    // Get the initial workspace name using the window we created
    let initial_workspace = match parse(&exec("activewindow -j")?) {
        Ok(value) => match value["workspace"]["name"].as_str() {
            Some(name) => name.to_string(),
            None => {
                return Err(Error::Generic(format!(
                    "No workspace name found: {:?}",
                    value
                )))
            }
        },

        Err(error) => return Err(error),
    };
    sleep(Duration::from_millis(MILLIS_TO_WAIT));

    // Minimize the window
    MinimizeCommand {
        active: true,
        all: false,
    }
    .command()?;

    let destiny_workspace = match parse(&exec("clients -j")?) {
        Ok(value) => match value.as_array() {
            Some(clients) => {
                let wanted_client = clients
                    .iter()
                    .filter(|c| match c["address"].as_str() {
                        Some(address) => {
                            address.chars().filter(|c| *c != '"').collect::<String>() == new_window
                        }
                        None => false,
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if wanted_client.len() != 1 {
                    return Err(Error::Generic(format!(
                        "Found various clients with same address: {:?}",
                        wanted_client
                    )));
                }
                match wanted_client[0]["workspace"]["name"].as_str() {
                    Some(workspace) => workspace.chars().filter(|c| *c != '"').collect::<String>(),
                    None => {
                        return Err(Error::Generic(format!(
                            "No workspace name found: {:?}",
                            wanted_client[0]
                        )))
                    }
                }
            }
            None => {
                return Err(Error::Generic(format!(
                    "No clients found after minimize: {:?}",
                    value
                )))
            }
        },
        Err(error) => return Err(error),
    };

    debug_assert!(destiny_workspace.contains(&initial_workspace));

    Ok(())
}
