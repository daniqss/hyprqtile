use hyprqtile::{commands::ToggleCommand, prelude::*};
use std::{thread::sleep, time::Duration};
mod utils;
use utils::*;

#[test]
fn test_toggle_command() -> Result<()> {
    // create window that we'll use to get data
    let window_in_special = create_window()?;

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

    // Move the window to the special workspace to check workspace toggling
    move_window_to(format!("special:{}", initial_workspace), &window_in_special)?;
    sleep(Duration::from_millis(MILLIS_TO_WAIT));

    // Toggle to the special workspace
    ToggleCommand {}.command()?;
    sleep(Duration::from_millis(MILLIS_TO_WAIT));

    // Get the new workspace name using the window that we've moved
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
