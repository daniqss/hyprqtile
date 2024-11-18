#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error(transparent)]
    Hyprland(#[from] hyprland::shared::HyprError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "tests")]
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
