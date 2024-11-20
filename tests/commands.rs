#[cfg(feature = "tests")]
mod tests {
    use hyprqtile::{
        commands::{MaximizeCommand, MinimizeCommand, ToggleCommand, WorkspaceCommand},
        prelude::*,
    };
    use serde_json::{from_slice, Value};
    use std::{collections::HashSet, thread::sleep, time::Duration};
    const MILLIS_TO_WAIT: u64 = 200;

    fn exec(args: &str) -> Result<Vec<u8>> {
        match std::process::Command::new("hyprctl")
            .args(args.split(' '))
            .output()
        {
            Ok(output) => Ok(output.stdout),
            Err(error) => Err(error.into()),
        }
    }

    fn parse(raw: &Vec<u8>) -> Result<Value> {
        match from_slice(raw) {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::SerdeJson(error)),
        }
    }

    fn create_window() -> Result<String> {
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

    fn move_window_to(workspace: String, address: &String) -> Result<()> {
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

    #[test]
    fn test_toggle_command() -> Result<()> {
        let window_in_special = create_window()?;

        sleep(Duration::from_millis(MILLIS_TO_WAIT));
        // Get the initial workspace name
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
        // Move the window to the special workspace to check workspace toggling
        // Toggle to the special workspace

        sleep(Duration::from_millis(MILLIS_TO_WAIT));
        move_window_to(format!("special:{}", initial_workspace), &window_in_special)?;
        sleep(Duration::from_millis(MILLIS_TO_WAIT));
        ToggleCommand {}.command()?;
        sleep(Duration::from_millis(MILLIS_TO_WAIT));

        // Get the new workspace name
        let new_workspace = match parse(&exec("activewindow -j")?) {
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

        debug_assert!(new_workspace.contains(&initial_workspace));

        Ok(())
    }
}
