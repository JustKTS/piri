use anyhow::{Context, Result};
use async_trait::async_trait;
use log::{debug, info};
use std::collections::HashMap;

use crate::config::Config;
use crate::ipc::IpcRequest;
use crate::niri::NiriIpc;
use crate::plugins::window_utils::{self, get_focused_window};
use crate::plugins::FromConfig;

/// Runtime-only plugin: marks are not persisted to disk.
#[derive(Debug, Clone, Default)]
pub struct MarkPluginConfig;

impl FromConfig for MarkPluginConfig {
    fn from_config(_config: &Config) -> Option<Self> {
        Some(Self)
    }
}

pub struct MarkPlugin {
    niri: NiriIpc,
    /// Mark name → window id
    marks: HashMap<String, u64>,
}

impl MarkPlugin {
    fn new(niri: NiriIpc) -> Self {
        info!("Mark plugin initialized");
        Self {
            niri,
            marks: HashMap::new(),
        }
    }

    async fn bind_focused(&mut self, name: &str) -> Result<()> {
        let window = get_focused_window(&self.niri).await?;
        debug!("Mark '{}' → window {}", name, window.id);
        self.marks.insert(name.to_string(), window.id);
        Ok(())
    }

    /// If `name` points to a live window, focus it; otherwise store the current focus under `name`.
    async fn toggle(&mut self, name: &str) -> Result<()> {
        let focus_existing = match self.marks.get(name).copied() {
            Some(id) => window_utils::window_exists(&self.niri, id).await?,
            None => false,
        };

        if focus_existing {
            let id = self
                .marks
                .get(name)
                .copied()
                .context("internal: mark disappeared after existence check")?;
            window_utils::focus_window(self.niri.clone(), id).await?;
        } else {
            self.bind_focused(name).await?;
        }
        Ok(())
    }

    fn delete(&mut self, name: &str) {
        self.marks.remove(name);
    }

    async fn add(&mut self, name: &str) -> Result<()> {
        self.bind_focused(name).await
    }
}

#[async_trait]
impl crate::plugins::Plugin for MarkPlugin {
    type Config = MarkPluginConfig;

    fn new(niri: NiriIpc, _config: Self::Config) -> Self {
        Self::new(niri)
    }

    async fn update_config(&mut self, _config: Self::Config) -> Result<()> {
        Ok(())
    }

    async fn handle_ipc_request(&mut self, request: &IpcRequest) -> Result<Option<Result<()>>> {
        match request {
            IpcRequest::MarkToggle { name } => {
                info!("Mark toggle: {}", name);
                self.toggle(name).await?;
                Ok(Some(Ok(())))
            }
            IpcRequest::MarkDelete { name } => {
                info!("Mark delete: {}", name);
                self.delete(name);
                Ok(Some(Ok(())))
            }
            IpcRequest::MarkAdd { name } => {
                info!("Mark add: {}", name);
                self.add(name).await?;
                Ok(Some(Ok(())))
            }
            _ => Ok(None),
        }
    }
}
