use hyprland::data::{Client, Clients, Monitor, Monitors};
use hyprland::dispatch::{
    Dispatch, DispatchType as DT, MonitorIdentifier, WorkspaceIdentifier,
    WorkspaceIdentifierWithSpecial,
};
use hyprland::shared::{Address, HyprData, HyprDataActiveOptional as _, HyprDataVec};
use std::fmt::Display;

pub mod error;
pub mod prelude;
pub use prelude::*;

pub struct MonitorsResult {
    pub active_monitor: i128,
    pub passive_monitor: Option<i128>,
    pub monitors: Vec<Monitor>,
}

impl MonitorsResult {
    fn get(workspace_id: i32) -> Result<Self> {
        let monitors = Monitors::get()?.to_vec();
        let mut active_monitor = 1;
        let mut passive_monitor = None;

        for monitor in &monitors {
            if monitor.focused {
                active_monitor = monitor.id;
                continue;
            }
            if monitor.active_workspace.id == workspace_id {
                passive_monitor = Some(monitor.id);
            }
        }

        Ok(Self {
            active_monitor,
            passive_monitor,
            monitors: monitors.to_vec(),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Workspace {
    Id(i32),
    Special(i32),
}

impl Into<i32> for Workspace {
    fn into(self) -> i32 {
        match self {
            Workspace::Id(id) => id,
            Workspace::Special(id) => id,
        }
    }
}

impl Into<String> for Workspace {
    fn into(self) -> String {
        match self {
            Workspace::Id(id) => id.to_string(),
            Workspace::Special(id) => id.to_string(),
        }
    }
}

impl Display for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Workspace::Id(id) => write!(f, "{}", id),
            Workspace::Special(id) => write!(f, "{}", id),
        }
    }
}

pub fn move_workspace_to(workspace_id: i32) -> Result<()> {
    let monitors = MonitorsResult::get(workspace_id)?;

    if monitors.monitors.len() == 1 {
        switch_to_workspace(workspace_id, None)?;
    }

    match monitors.passive_monitor {
        Some(passive_monitor_id) => {
            swap_active_workspace(monitors.active_monitor, passive_monitor_id)
        }
        None => switch_to_workspace(workspace_id, Some(monitors.active_monitor)),
    }
}

pub fn toggle_special_workspace() -> Result<()> {
    Ok(Dispatch::call(DT::ToggleSpecialWorkspace(
        match get_current_workspace() {
            Ok(workspace) => Some(workspace.to_string()),
            Err(_) => None,
        },
    ))?)
}

pub fn get_active_workspace_windows_addresses() -> Result<Vec<String>> {
    let workspace = get_current_workspace()?;
    Ok(Clients::get()?
        .iter()
        .filter(|c| c.workspace.id == Into::<i32>::into(workspace.clone()))
        .map(|c| c.address.to_string())
        .collect())
}

pub fn reset_submap() -> Result<()> {
    Ok(Dispatch::call(DT::Custom("submap", "reset"))?)
}

pub fn move_window(workspace: Workspace, window_address: Option<&String>) -> Result<()> {
    Ok(Dispatch::call(DT::MoveToWorkspaceSilent(
        match workspace {
            Workspace::Id(id) => WorkspaceIdentifierWithSpecial::Id(id),
            Workspace::Special(special) => {
                // we are forced to leak the string because it wants a static string slice
                let leaked_string: &'static str = Box::leak(special.to_string().into_boxed_str());
                WorkspaceIdentifierWithSpecial::Special(Some(leaked_string))
            }
        },
        match window_address {
            Some(address) => Some(hyprland::dispatch::WindowIdentifier::Address(Address::new(
                address,
            ))),
            None => None,
        },
    ))?)
}

pub fn get_current_workspace_id() -> Result<i32> {
    let active_window = Client::get_active()?;
    match active_window {
        Some(window) => Ok(window.workspace.id),
        None => Err(Error::Generic("No active window".to_string())),
    }
}

pub fn get_current_workspace() -> Result<Workspace> {
    let workspace_id = get_current_workspace_id()?;
    match workspace_id < 0 {
        true => Ok(Workspace::Special(workspace_id)),
        false => Ok(Workspace::Id(workspace_id)),
    }
}

pub fn swap_active_workspace(active_monitor_id: i128, passive_monitor_id: i128) -> Result<()> {
    let active_monitor = MonitorIdentifier::Id(active_monitor_id);
    let passive_monitor = MonitorIdentifier::Id(passive_monitor_id);
    Ok(Dispatch::call(DT::SwapActiveWorkspaces(
        active_monitor,
        passive_monitor,
    ))?)
}

pub fn switch_to_workspace(workspace_id: i32, active_monitor_id: Option<i128>) -> Result<()> {
    let workspace = WorkspaceIdentifierWithSpecial::Id(workspace_id);
    if let Some(active_monitor_id) = active_monitor_id {
        let workspace = WorkspaceIdentifier::Id(workspace_id);
        let active_monitor = MonitorIdentifier::Id(active_monitor_id);
        Dispatch::call(DT::MoveWorkspaceToMonitor(workspace, active_monitor))?;
    }
    match Dispatch::call(DT::Workspace(workspace)) {
        Ok(_) => Ok(()),
        Err(_) => {
            // If the workspace doesn't exist (non windows in it unless it's configured as persistent)
            // it will return an error, but the call create the workspace and switch to it
            // so we can ignore the error
            Ok(())
        }
    }
}
