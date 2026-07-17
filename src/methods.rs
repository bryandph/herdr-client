//! Typed convenience methods over [`Connection`] for the surface workmux drives.
//!
//! Every method's *params* type comes straight from herdr's schema (reliable).
//! *Result* typing is pinned to a concrete generated type only where herdr's
//! schema names it unambiguously — `get`-style queries (→ the matching `*Info`)
//! and methods with a dedicated `*Result` def. Methods whose result shape varies
//! or isn't yet confirmed return [`serde_json::Value`]; a caller that knows the
//! shape can always use [`Connection::call`] to deserialize into its own type.

use serde_json::Value;

use crate::error::Result;
use crate::transport::Pong;
use crate::{request, success_response, Connection};

impl Connection {
    // --- server ---------------------------------------------------------

    /// `ping` — handshake; prefer [`Connection::check_protocol`] to also verify
    /// the protocol version.
    pub fn ping(&mut self) -> Result<Pong> {
        self.call("ping", &serde_json::json!({}))
    }

    // --- panes ----------------------------------------------------------

    /// `pane.split` — split a pane; returns herdr's raw result (new pane info).
    pub fn pane_split(&mut self, params: &request::PaneSplitParams) -> Result<Value> {
        self.call_value("pane.split", params)
    }

    /// `pane.get` — info for a specific pane.
    pub fn pane_get(&mut self, params: &request::PaneTarget) -> Result<success_response::PaneInfo> {
        self.call("pane.get", params)
    }

    /// `pane.current` — info for the calling pane.
    pub fn pane_current(
        &mut self,
        params: &request::PaneCurrentParams,
    ) -> Result<success_response::PaneInfo> {
        self.call("pane.current", params)
    }

    /// `pane.read` — read pane contents.
    pub fn pane_read(
        &mut self,
        params: &request::PaneReadParams,
    ) -> Result<success_response::PaneReadResult> {
        self.call("pane.read", params)
    }

    /// `pane.list` — list panes.
    pub fn pane_list(&mut self, params: &request::PaneListParams) -> Result<Value> {
        self.call_value("pane.list", params)
    }

    /// `pane.send_text` — type text into a pane.
    pub fn pane_send_text(&mut self, params: &request::PaneSendTextParams) -> Result<()> {
        self.call_ack("pane.send_text", params)
    }

    /// `pane.send_keys` — send named keys to a pane.
    pub fn pane_send_keys(&mut self, params: &request::PaneSendKeysParams) -> Result<()> {
        self.call_ack("pane.send_keys", params)
    }

    /// `pane.send_input` — send raw input bytes to a pane.
    pub fn pane_send_input(&mut self, params: &request::PaneSendInputParams) -> Result<()> {
        self.call_ack("pane.send_input", params)
    }

    /// `pane.close` — close a pane.
    pub fn pane_close(&mut self, params: &request::PaneTarget) -> Result<()> {
        self.call_ack("pane.close", params)
    }

    /// `pane.rename` — rename a pane.
    pub fn pane_rename(&mut self, params: &request::PaneRenameParams) -> Result<()> {
        self.call_ack("pane.rename", params)
    }

    /// `pane.zoom` — toggle/set pane zoom.
    pub fn pane_zoom(
        &mut self,
        params: &request::PaneZoomParams,
    ) -> Result<success_response::PaneZoomResult> {
        self.call("pane.zoom", params)
    }

    /// `pane.report_agent` — report the agent running in a pane.
    pub fn pane_report_agent(&mut self, params: &request::PaneReportAgentParams) -> Result<()> {
        self.call_ack("pane.report_agent", params)
    }

    // --- workspaces -----------------------------------------------------

    /// `workspace.create` — create a workspace; returns herdr's raw result.
    pub fn workspace_create(&mut self, params: &request::WorkspaceCreateParams) -> Result<Value> {
        self.call_value("workspace.create", params)
    }

    /// `workspace.get` — info for a workspace.
    pub fn workspace_get(
        &mut self,
        params: &request::WorkspaceTarget,
    ) -> Result<success_response::WorkspaceInfo> {
        self.call("workspace.get", params)
    }

    /// `workspace.list` — list workspaces.
    pub fn workspace_list(&mut self) -> Result<Value> {
        self.call_value("workspace.list", &serde_json::json!({}))
    }

    /// `workspace.close` — close a workspace.
    pub fn workspace_close(&mut self, params: &request::WorkspaceTarget) -> Result<()> {
        self.call_ack("workspace.close", params)
    }

    /// `workspace.rename` — rename a workspace.
    pub fn workspace_rename(&mut self, params: &request::WorkspaceRenameParams) -> Result<()> {
        self.call_ack("workspace.rename", params)
    }

    // --- tabs -----------------------------------------------------------

    /// `tab.create` — create a tab; returns herdr's raw result.
    pub fn tab_create(&mut self, params: &request::TabCreateParams) -> Result<Value> {
        self.call_value("tab.create", params)
    }

    /// `tab.get` — info for a tab.
    pub fn tab_get(&mut self, params: &request::TabTarget) -> Result<success_response::TabInfo> {
        self.call("tab.get", params)
    }

    /// `tab.list` — list tabs.
    pub fn tab_list(&mut self, params: &request::TabListParams) -> Result<Value> {
        self.call_value("tab.list", params)
    }

    /// `tab.close` — close a tab.
    pub fn tab_close(&mut self, params: &request::TabTarget) -> Result<()> {
        self.call_ack("tab.close", params)
    }

    // --- worktrees ------------------------------------------------------

    /// `worktree.create` — create a worktree; returns herdr's raw result.
    pub fn worktree_create(&mut self, params: &request::WorktreeCreateParams) -> Result<Value> {
        self.call_value("worktree.create", params)
    }

    /// `worktree.open` — open a worktree; returns herdr's raw result.
    pub fn worktree_open(&mut self, params: &request::WorktreeOpenParams) -> Result<Value> {
        self.call_value("worktree.open", params)
    }

    /// `worktree.remove` — remove a worktree.
    pub fn worktree_remove(&mut self, params: &request::WorktreeRemoveParams) -> Result<()> {
        self.call_ack("worktree.remove", params)
    }

    /// `worktree.list` — list worktrees.
    pub fn worktree_list(&mut self, params: &request::WorktreeListParams) -> Result<Value> {
        self.call_value("worktree.list", params)
    }

    // --- events ---------------------------------------------------------

    /// `events.wait` — block until a matching event (e.g. an agent status
    /// change); returns herdr's raw event payload.
    pub fn events_wait(&mut self, params: &request::EventsWaitParams) -> Result<Value> {
        self.call_value("events.wait", params)
    }
}
