use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::Path,
};
use typeshare::typeshare;

use super::d_window_state::DWindowState;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DAppState {
    pub windows: HashMap<String, DWindowState>,
}

impl DAppState {
    pub fn read_from(file_path: &str) -> Result<Self> {
        let path = Path::new(file_path);

        let mut content = String::new();
        File::open(path)
            .with_context(|| format!("Failed to open config file at {:?}", path))?
            .read_to_string(&mut content)
            .context("Failed to read config file")?;

        serde_json::from_str(&content).context("Failed to deserialize config JSON into AppState")
    }

    pub fn write_to(&self, file_path: &str) -> Result<()> {
        let path = Path::new(file_path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {:?}", parent))?;
        }

        let json =
            serde_json::to_string_pretty(self).context("Failed to serialize AppState to JSON")?;

        File::create(path)
            .with_context(|| format!("Failed to create config file at {:?}", path))?
            .write_all(json.as_bytes())
            .context("Failed to write to config file")?;

        Ok(())
    }

    pub fn get_window_state(&mut self, window_id: &str) -> Result<&DWindowState> {
        self.windows
            .get(window_id)
            .ok_or_else(|| anyhow!("Window ID not found"))
    }

    pub fn get_window_state_mut(&mut self, window_id: &str) -> Result<&mut DWindowState> {
        self.windows
            .get_mut(window_id)
            .ok_or_else(|| anyhow!("Window ID not found"))
    }
}
