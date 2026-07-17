//! Wire-shape round-trips against herdr's exact JSON, without a live socket.
//!
//! These lock the two things the transport must get byte-right: the request
//! frame it emits, and its ability to decode herdr's real response payloads
//! into the generated types.

use herdr_client::request::{PaneSplitParams, SplitDirection};
use herdr_client::transport::Pong;

/// A `pane.split` request must serialize to herdr's exact frame shape.
#[test]
fn pane_split_frame_roundtrips() {
    let params = PaneSplitParams {
        direction: SplitDirection::Right,
        target_pane_id: Some("pane_7".to_string()),
        workspace_id: None,
        cwd: None,
        env: Default::default(),
        focus: false,
        ratio: None,
    };

    // The transport builds `{"id","method","params"}`; assert the whole frame.
    let frame = serde_json::json!({
        "id": "req_1",
        "method": "pane.split",
        "params": &params,
    });

    // Optional/empty fields are skipped by the generated `serde` attrs; only
    // `direction`, `focus` (always emitted), and the set `target_pane_id` remain.
    let expected = serde_json::json!({
        "id": "req_1",
        "method": "pane.split",
        "params": {
            "direction": "right",
            "focus": false,
            "target_pane_id": "pane_7",
        }
    });

    assert_eq!(frame, expected);

    // And the params round-trip back to the typed struct.
    let back: PaneSplitParams = serde_json::from_value(frame["params"].clone()).unwrap();
    assert_eq!(back, params);
}

/// herdr's `pong` result must decode into the handshake type.
#[test]
fn pong_decodes() {
    let raw = serde_json::json!({
        "type": "pong",
        "version": "0.42.0",
        "protocol": herdr_client::PROTOCOL,
        "capabilities": { "live_handoff": true }
    });
    let pong: Pong = serde_json::from_value(raw).unwrap();
    assert_eq!(pong.kind, "pong");
    assert_eq!(pong.protocol, herdr_client::PROTOCOL);
    assert_eq!(pong.version, "0.42.0");
}
