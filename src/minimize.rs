use crate::prelude::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct MinimizeCommand {
    /// Toggle to the workspace special's
    #[arg(short, long, conflicts_with = "all", conflicts_with = "active")]
    pub toggle: bool,
    /// Minimizes/maximizes the active window
    #[arg(short, long, conflicts_with = "toggle", conflicts_with = "all")]
    pub active: bool,
    /// Minimizes/maximizes all windows in the active workspace
    #[arg(long, conflicts_with = "toggle", conflicts_with = "active")]
    pub all: bool,
}

impl MinimizeCommand {
    pub fn command(self) -> Result<()> {
        match self {
            Self { toggle: true, .. } => self.toggle(),
            Self { active: true, .. } => self.active(),
            Self { all: true, .. } => self.all(),
            _ => Ok(()),
        }
    }

    fn toggle(self) -> Result<()> {
        Ok(())
    }

    fn active(self) -> Result<()> {
        Ok(())
    }

    fn all(self) -> Result<()> {
        Ok(())
    }
}
