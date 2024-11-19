#[cfg(feature = "tests")]
mod tests {
    use std::collections::HashSet;

    use hyprqtile::{
        commands::{MaximizeCommand, MinimizeCommand, ToggleCommand, WorkspaceCommand},
        prelude::*,
    };
    use serde_json::{from_slice, Value};
    type Address = String;

    fn exec(args: &str) -> Result<Vec<u8>> {
        match std::process::Command::new("hyprctl")
            .args(args.lines())
            .output()
        {
            Ok(output) => Ok(output.stdout),
            Err(error) => Err(error.into()),
        }
    }

    fn parse(raw: Vec<u8>) -> Result<Value> {
        println!("hola desde parse");
        let value = match from_slice(&raw) {
            Ok(value) => value,
            Err(error) => return Err(Error::SerdeJson(error)),
        };
        println!("hola desde despues de parse");
        Ok(value)
    }

    fn create_window() -> Result<Address> {
        let Some(old_clients) = parse(exec("clients -j")?)?.as_array().cloned() else {
            return Err(Error::Generic("No clients found".to_string()));
        };
        let old_clients: HashSet<&Value> = old_clients.iter().collect();

        exec(
            format!(
                "dispatch exec {}",
                // search for TERM, if not set use kitty as it's the default terminal for hyprland
                std::env::var("TERM").unwrap_or_else(|_| "kitty".to_string())
            )
            .as_str(),
        )?;
        let Some(new_clients) = parse(exec("clients -j")?)?.as_array().cloned() else {
            return Err(Error::Generic("No clients found".to_string()));
        };
        let new_clients: HashSet<&Value> = new_clients.iter().collect();
        let differences: Vec<_> = new_clients.difference(&old_clients).cloned().collect();
        if differences.len() != 1 {
            return Err(Error::Generic("No new client found".to_string()));
        }
        match differences[0]["address"].as_str() {
            Some(address) => Ok(address.to_string()),
            None => Err(Error::Generic(
                "Error getting founded new client address".to_string(),
            )),
        }
    }

    fn move_window_to(workspace: String, address: &Address) -> Result<()> {
        exec(format!("movetoworkspacesilent {},{}", workspace, address).as_str())?;
        Ok(())
    }

    #[test]
    fn test_toggle_command() -> Result<()> {
        println!("hola wei");
        let _window_in_normal = create_window()?;
        let window_in_special = create_window()?;

        // Get the initial workspace name
        let initial_name = parse(exec("activewindow -j")?)?["workspace"]["name"]
            .as_str()
            .ok_or(Error::Generic("No workspace name found".to_string()))?
            .to_string();

        // Move the window to the special workspace to check workspace toggling
        move_window_to(format!("special:{}", initial_name), &window_in_special)?;
        // Toggle to the special workspace
        let toggle_command = ToggleCommand {};
        toggle_command.command()?;

        // Get the new workspace name
        let new_name = parse(exec("activewindow -j")?)?["workspace"]["name"]
            .as_str()
            .ok_or(Error::Generic("No workspace name found".to_string()))?
            .to_string();

        debug_assert!(new_name.contains(&initial_name));

        Ok(())
    }
}
