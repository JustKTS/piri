use anyhow::{Context, Result};
use async_trait::async_trait;
use log::{debug, info, warn};
use niri_ipc::Event;

use crate::config::Config;
use crate::ipc::IpcRequest;
use crate::niri::NiriIpc;
use crate::plugins::window_utils;
use crate::plugins::FromConfig;

#[derive(Debug, Clone, Default)]
pub struct StickyPluginConfig;

impl FromConfig for StickyPluginConfig {
    fn from_config(_config: &Config) -> Option<Self> {
        Some(Self)
    }
}

pub struct StickyPlugin {
    niri: NiriIpc,
    sticky_window_id: Option<u64>,
    cross_monitor: bool,
}

impl StickyPlugin {
    fn new(niri: NiriIpc) -> Self {
        info!("Sticky plugin initialized");
        Self {
            niri,
            sticky_window_id: None,
            cross_monitor: false,
        }
    }

    async fn add(&mut self, cross: bool) -> Result<()> {
        let window = window_utils::get_focused_window(&self.niri).await?;
        if !window.floating {
            anyhow::bail!("Focused window is not floating. Sticky only supports floating windows.");
        }

        self.sticky_window_id = Some(window.id);
        self.cross_monitor = cross;
        info!(
            "Sticky add: window_id={}, cross_monitor={}",
            window.id, cross
        );
        Ok(())
    }

    fn delete(&mut self) {
        self.sticky_window_id = None;
        self.cross_monitor = false;
        info!("Sticky deleted");
    }

    async fn follow_focused_workspace(&mut self) -> Result<()> {
        let Some(window_id) = self.sticky_window_id else {
            return Ok(());
        };

        let windows = self.niri.get_windows().await?;
        let window = match windows.into_iter().find(|w| w.id == window_id) {
            Some(w) => w,
            None => {
                warn!(
                    "Sticky window {} no longer exists, clearing state",
                    window_id
                );
                self.delete();
                return Ok(());
            }
        };

        if !window.floating {
            warn!(
                "Sticky window {} is no longer floating, clearing sticky state",
                window_id
            );
            self.delete();
            return Ok(());
        }

        if self.cross_monitor {
            self.niri.move_floating_window(window_id).await?;
            return Ok(());
        }

        let focused_workspace = self.niri.get_focused_workspace().await?;
        let workspaces = self.niri.get_workspaces_for_mapping().await?;

        let target_workspace = workspaces
            .iter()
            .find(|ws| ws.idx.to_string() == focused_workspace.name)
            .context("Focused workspace not found in workspace list")?;

        if let Some(window_workspace_id) = window.workspace_id {
            if let Some(source_workspace) =
                workspaces.iter().find(|ws| ws.id == window_workspace_id)
            {
                if source_workspace.output != target_workspace.output {
                    debug!(
                        "Sticky skip move: cross=false and output differs (from {:?} to {:?})",
                        source_workspace.output, target_workspace.output
                    );
                    return Ok(());
                }
            }
        }

        self.niri.move_window_to_workspace(window_id, &focused_workspace.name).await?;
        Ok(())
    }
}

#[async_trait]
impl crate::plugins::Plugin for StickyPlugin {
    type Config = StickyPluginConfig;

    fn new(niri: NiriIpc, _config: Self::Config) -> Self {
        Self::new(niri)
    }

    async fn update_config(&mut self, _config: Self::Config) -> Result<()> {
        Ok(())
    }

    async fn handle_ipc_request(&mut self, request: &IpcRequest) -> Result<Option<Result<()>>> {
        match request {
            IpcRequest::StickyAdd { cross } => {
                self.add(*cross).await?;
                Ok(Some(Ok(())))
            }
            IpcRequest::StickyDelete => {
                self.delete();
                Ok(Some(Ok(())))
            }
            _ => Ok(None),
        }
    }

    async fn handle_event(&mut self, event: &Event, _niri: &NiriIpc) -> Result<()> {
        if let Event::WorkspaceActivated { focused: true, .. } = event {
            self.follow_focused_workspace().await?;
        }
        Ok(())
    }

    fn is_interested_in_event(&self, event: &Event) -> bool {
        matches!(event, Event::WorkspaceActivated { focused: true, .. })
    }
}
