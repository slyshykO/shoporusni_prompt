use anyhow::{Context, Result};
use log::debug;
use std::fs::DirBuilder;
use std::path::PathBuf;

pub fn config_dir() -> Result<PathBuf> {
    let cfg_dir = dirs::config_dir()
        .unwrap_or_default()
        .join("shoporusni");
    debug!("Config dir: {:?}", cfg_dir);
    // Create a new config dir if it doesn't exist
    DirBuilder::new()
        .recursive(true)
        .create(&cfg_dir)
        .context("Creating a new config directory")?;
    Ok(cfg_dir)
}
