//! Newline-delimited JSON socket transport for herdr.
//!
//! herdr speaks a line-per-message JSON protocol over a Unix socket: the client
//! writes `{"id","method","params"}` frames and reads back `{"id","result"}` or
//! `{"id","error"}`. This module owns the socket, id generation, and response
//! correlation; the typed method surface (see the `methods` module) is a thin
//! layer of param/result types on top.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::error::{Error, Result};

/// Environment variable herdr exports into panes, holding the control socket.
pub const SOCKET_ENV: &str = "HERDR_SOCKET_PATH";

/// A synchronous connection to a herdr server.
pub struct Connection {
    reader: BufReader<UnixStream>,
    writer: UnixStream,
    next_id: u64,
}

/// Response to `ping` — herdr's handshake, carrying its live protocol version.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Pong {
    /// Always `"pong"`.
    #[serde(rename = "type")]
    pub kind: String,
    /// herdr server version string.
    pub version: String,
    /// Wire protocol version the server speaks.
    pub protocol: u32,
}

impl Connection {
    /// Connect using `$HERDR_SOCKET_PATH` (set by herdr inside every pane).
    pub fn from_env() -> Result<Self> {
        let path = std::env::var_os(SOCKET_ENV).ok_or(Error::NoSocketPath)?;
        Self::connect(path)
    }

    /// Connect to a herdr control socket at `path`.
    pub fn connect(path: impl AsRef<Path>) -> Result<Self> {
        let stream = UnixStream::connect(path)?;
        let reader = BufReader::new(stream.try_clone()?);
        Ok(Self {
            reader,
            writer: stream,
            next_id: 1,
        })
    }

    /// Send a request and deserialize its `result` payload into `R`.
    ///
    /// `R` is chosen by the caller because herdr's schema does not discriminate
    /// result shape by method at the type level; the typed methods in the
    /// `methods` module pin sensible `R`s for herdr's documented results.
    pub fn call<P: Serialize, R: DeserializeOwned>(
        &mut self,
        method: &str,
        params: &P,
    ) -> Result<R> {
        let result = self.call_value(method, params)?;
        Ok(serde_json::from_value(result)?)
    }

    /// Send a request and discard the result payload (for ack-style methods).
    pub fn call_ack<P: Serialize>(&mut self, method: &str, params: &P) -> Result<()> {
        self.call_value(method, params).map(|_| ())
    }

    /// Send a request and return its raw `result` JSON.
    ///
    /// This is the core of the transport: it builds the frame, writes it
    /// newline-delimited, and reads response lines until one carries our id —
    /// skipping any interleaved traffic that isn't the reply we're waiting for.
    pub fn call_value<P: Serialize>(&mut self, method: &str, params: &P) -> Result<Value> {
        let id = format!("req_{}", self.next_id);
        self.next_id += 1;

        let frame = build_frame(&id, method, params)?;
        let mut line = serde_json::to_vec(&frame)?;
        line.push(b'\n');
        self.writer.write_all(&line)?;
        self.writer.flush()?;

        loop {
            let mut buf = String::new();
            if self.reader.read_line(&mut buf)? == 0 {
                return Err(Error::Closed);
            }
            let line = buf.trim_end();
            if line.is_empty() {
                continue;
            }
            let resp: Value = serde_json::from_str(line)?;
            if resp.get("id").and_then(Value::as_str) != Some(id.as_str()) {
                continue; // not our reply (e.g. an event frame) — keep reading
            }
            if let Some(err) = resp.get("error") {
                return Err(Error::Herdr {
                    code: err
                        .get("code")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    message: err
                        .get("message")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                });
            }
            return Ok(resp.get("result").cloned().unwrap_or(Value::Null));
        }
    }

    /// Handshake: `ping` the server and verify its protocol matches
    /// [`crate::PROTOCOL`], failing loud on mismatch rather than risking a
    /// silent mis-decode later.
    pub fn check_protocol(&mut self) -> Result<Pong> {
        let pong: Pong = self.call("ping", &serde_json::json!({}))?;
        if pong.protocol != crate::PROTOCOL {
            return Err(Error::ProtocolMismatch {
                client: crate::PROTOCOL,
                server: pong.protocol,
            });
        }
        Ok(pong)
    }
}

/// Build a herdr request frame `{"id","method","params"}`.
///
/// Factored out so the exact wire shape can be asserted in tests without a live
/// socket.
pub(crate) fn build_frame<P: Serialize>(id: &str, method: &str, params: &P) -> Result<Value> {
    Ok(serde_json::json!({
        "id": id,
        "method": method,
        "params": params,
    }))
}
