//! Types generated from herdr's published JSON Schema — **do not edit by hand**.
//!
//! Produced by `scripts/regen.sh` from `schema/herdr-api.schema.json`
//! (herdr `protocol` [`protocol::PROTOCOL`]). herdr's schema is a container of
//! five self-contained sub-schemas with no cross-references; each is flattened
//! and fed through `typify` independently, landing in the like-named module
//! below. Same-named-but-differing defs across sub-schemas (e.g. `PaneInfo`,
//! `EventData`) are the reason they are kept in separate modules rather than a
//! merged namespace.
//!
//! Regenerate and check for drift with `scripts/regen.sh`; the committed output
//! is the source of truth reviewers see when herdr's protocol moves.

// Generated code is not held to the crate's hand-written lint bar.
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(rustdoc::all)]

pub mod protocol;

pub mod error_response;
pub mod event;
pub mod request;
pub mod subscription_event;
pub mod success_response;
