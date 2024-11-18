use clap::Args;
use hyprqtile::toggle_special_workspace;
use hyprqtile::Result;

/// Toggle from the current workspace to its special workspace
#[derive(Args, Debug)]
#[command()]
pub struct ToggleCommand {}
impl ToggleCommand {
    pub fn command(self) -> Result<()> {
        toggle_special_workspace()
    }
}

#[cfg(test)]
mod tests {
    use hyprqtile::commands::ToggleCommand;
    use serde_json::{from_slice, Value};

    #[test]
    fn test_toggle_command() -> core::result::Result<(), Box<dyn std::error::Error>> {
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

        let new_name = new_response["workspace"]["name"]
            .as_str()
            .unwrap()
            .to_string();

        debug_assert!(new_name.contains(&initial_name));

        Ok(())
    }
}
