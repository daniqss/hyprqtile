use hyprland::data::*;
use hyprland::dispatch::{
    Dispatch, DispatchType as DT, MonitorIdentifier, WorkspaceIdentifier,
    WorkspaceIdentifierWithSpecial,
};

use hyprland::prelude::*;
use hyprland::shared::HyprError;
use hyprland::Result;

pub struct MonitorsResult {
    pub active_monitor: i128,
    pub passive_monitor: Option<i128>,
    pub monitors: Vec<Monitor>,
}

impl MonitorsResult {
    pub fn get(workspace_id: i32) -> Result<Self> {
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

pub fn move_to(workspace_id: i32) -> Result<()> {
    let monitors = MonitorsResult::get(workspace_id)?;

    if monitors.monitors.len() == 1 {
        switch_to_workspace(workspace_id, None)?;
    };

    match monitors.passive_monitor {
        Some(passive_monitor_id) => {
            swap_active_workspace(monitors.active_monitor, passive_monitor_id)?
        }
        _ => switch_to_workspace(workspace_id, Some(monitors.active_monitor))?,
    }

    Ok(())
}

pub fn get_current_workspace() -> Result<i32> {
    let workspace = Workspace::get_active()?.id;
    Ok(workspace)
}

pub fn move_to_next() -> Result<()> {
    match get_current_workspace() {
        Ok(workspace) => move_to(workspace + 1),
        Err(hypr_error) => Err(hypr_error),
    }
}

pub fn move_to_previous() -> Result<()> {
    match get_current_workspace() {
        Ok(workspace) => move_to(workspace - 1),
        Err(hypr_error) => Err(hypr_error),
    }
}

pub fn swap_active_workspace(active_monitor_id: i128, passive_monitor_id: i128) -> Result<()> {
    let active_monitor = MonitorIdentifier::Id(active_monitor_id);
    let passive_monitor = MonitorIdentifier::Id(passive_monitor_id);
    Dispatch::call(DT::SwapActiveWorkspaces(active_monitor, passive_monitor))?;
    Ok(())
}

pub fn switch_to_workspace(workspace_id: i32, active_monitor_id: Option<i128>) -> Result<()> {
    let workspace = WorkspaceIdentifierWithSpecial::Id(workspace_id);
    if let Some(active_monitor_id) = active_monitor_id {
        let workspace = WorkspaceIdentifier::Id(workspace_id);
        let active_monitor = MonitorIdentifier::Id(active_monitor_id);

        match Dispatch::call(DT::MoveWorkspaceToMonitor(workspace, active_monitor)) {
            Err(e) => match e {
                HyprError::NotOkDispatch(val)
                    if val == "moveWorkspaceToMonitor workspace doesn't exist!".to_owned() =>
                {
                    ()
                }
                _ => return Err(e),
            },
            _ => (),
        };
    }

    match Dispatch::call(DT::Workspace(workspace)) {
        Ok(()) => Ok(()),
        Err(e) => match e {
            HyprError::NotOkDispatch(val)
                if val == "Previous workspace doesn't exist".to_owned() =>
            {
                Ok(())
            }
            _ => Err(e),
        },
    }
}
