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
#[derive(Debug, Clone)]
pub struct MarkPluginConfig {
    pub refocus: bool,
}

impl FromConfig for MarkPluginConfig {
    fn from_config(config: &Config) -> Option<Self> {
        Some(Self {
            refocus: config.piri.mark.refocus,
        })
    }
}

pub struct MarkPlugin {
    niri: NiriIpc,
    /// Mark name → window id
    marks: HashMap<String, u64>,
    /// Previous focused window id (for refocus feature)
    previous_window: Option<u64>,
    /// Enable refocus feature
    refocus: bool,
}

impl MarkPlugin {
    fn new(niri: NiriIpc, config: MarkPluginConfig) -> Self {
        info!("Mark plugin initialized (refocus: {})", config.refocus);
        Self {
            niri,
            marks: HashMap::new(),
            previous_window: None,
            refocus: config.refocus,
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

            // Try to get current focused window (may fail on empty workspace)
            if let Ok(current) = get_focused_window(&self.niri).await {
                if self.refocus && current.id == id {
                    if let Some(prev_id) = self.previous_window {
                        if window_utils::window_exists(&self.niri, prev_id).await? {
                            debug!("Refocusing to previous window {}", prev_id);
                            // Swap: set previous to current marked window for next toggle
                            self.previous_window = Some(id);
                            window_utils::focus_window(self.niri.clone(), prev_id).await?;
                            return Ok(());
                        }
                    }
                }

                debug!("Saving previous window {} before focusing mark", current.id);
                self.previous_window = Some(current.id);
            } else {
                // No focused window (empty workspace), clear previous_window
                debug!("No focused window, clearing previous_window");
                self.previous_window = None;
            }

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

    fn new(niri: NiriIpc, config: Self::Config) -> Self {
        Self::new(niri, config)
    }

    async fn update_config(&mut self, config: Self::Config) -> Result<()> {
        self.refocus = config.refocus;
        info!("Mark plugin updated (refocus: {})", self.refocus);
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
