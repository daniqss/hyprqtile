#[cfg(feature = "tests")]
mod tests {
    use hyprqtile::{
        commands::{MaximizeCommand, MinimizeCommand, ToggleCommand, WorkspaceCommand},
        prelude::*,
    };
    use serde_json::{from_slice, Value};

    #[test]
    fn test_toggle_command() -> Result<()> {
        let initial_response: Value = match std::process::Command::new("hyprctl")
            .arg("activewindow")
            .arg("-j")
            .output()
        {
            Ok(output) => match from_slice(&output.stdout) {
                Ok(response) => response,
                Err(error) => return Err(error.into()),
            },
            Err(error) => return Err(error.into()),
        };
        let initial_name = initial_response["workspace"]["name"]
            .as_str()
            .unwrap()
            .to_string();

        // Toggle to the special workspace
        let toggle_command = ToggleCommand {};
        toggle_command.command()?;

        let new_response: Value = match std::process::Command::new("hyprctl")
            .arg("activewindow")
            .arg("-j")
            .output()
        {
            Ok(output) => match serde_json::from_slice(&output.stdout) {
                Ok(response) => response,
                Err(error) => return Err(error.into()),
            },
            Err(error) => return Err(error.into()),
        };

        let new_name = match new_response["workspace"]["name"].as_str() {
            Some(name) => name.to_string(),
            None => {
                return Err(Error::Generic(
                    "No active window in the special name found".to_string(),
                ))
            }
        };

        debug_assert!(new_name.contains(&initial_name));

        Ok(())
    }
}
