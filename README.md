# herdr-client

Typed Rust client for [herdr](https://github.com/ogulcancelik/herdr)'s socket
API. herdr is an agent-aware terminal multiplexer whose CLI is a thin wrapper
over a newline-delimited JSON socket protocol; this crate is the typed
equivalent of that CLI, so a Rust program (notably the
[workmux](https://github.com/raine/workmux) herdr backend) can drive herdr over
the socket instead of shelling out to `herdr` and string-parsing stdout.

License: **MIT OR Apache-2.0**. herdr itself is AGPL-3.0 — this crate contains
**only** types generated from herdr's published schema plus a hand-written
transport, no herdr implementation source. It is a separate program speaking
herdr's documented protocol, which is the entire reason it is a distinct crate
rather than a fork/merge (a merged binary would be virally AGPL).

## Layout

| Path | What |
|------|------|
| `schema/herdr-api.schema.json` | Pinned copy of herdr's published schema (`herdr api schema --json`). Records the `protocol` / `schema_version` the checked-in types were generated against. |
| `src/generated/` | **Generated, do not edit.** One module per herdr sub-schema, produced by `scripts/regen.sh`. Committed (not `build.rs`) so builds are hermetic and protocol changes are reviewable diffs. |
| `src/generated/protocol.rs` | `PROTOCOL` / `SCHEMA_VERSION` consts lifted from the pinned schema, for fail-loud version checks. |
| `scripts/flatten.py` | Rewrites one sub-schema's absolute `$ref`s to root-relative so `typify` can consume it standalone; errors on any cross-sub-schema ref. |
| `scripts/regen.sh` | flatten → `cargo-typify` per sub-schema → `src/generated/*.rs`. |

## Regenerating (and drift detection)

```sh
nix develop        # cargo-typify + python3 + rust toolchain
./scripts/regen.sh # rewrites src/generated/*.rs from the pinned schema
git diff --exit-code src/generated   # clean = reproducible; non-empty = drift
```

CI runs exactly this: regeneration must reproduce the committed output with no
diff. To adopt a new herdr release, drop its `herdr api schema --json` over
`schema/herdr-api.schema.json`, re-run `regen.sh`, and review the diff — a
changed field type will surface as a compile break in this crate or its
consumers rather than a silent runtime mis-decode.

The schema is a container of five self-contained sub-schemas (`request`,
`success_response`, `error_response`, `event`, `subscription_event`) with **no
cross-references**. They are generated independently because several carry
same-named-but-differing defs (e.g. `PaneInfo`, `EventData`) that a merged
namespace would clobber.

## The typed veneer (why not use the generated `Request` enum)

`typify` renders herdr's top-level `Request` `oneOf` as an untagged
`Variant0..N` enum discriminated by a `const` `method` field. That is
unergonomic to construct and lossy on deserialize, so the client does **not**
expose it. Instead the hand-written transport consumes the individual params /
result structs from the generated modules directly and builds the wire frame
(`{"id": …, "method": "pane.split", "params": …}`) itself — which is also the
natural home for id generation, socket send, and response correlation. The
generated `Variant*` envelope types remain in `src/generated` (they are part of
the faithful schema translation and its drift check) but are not part of this
crate's public surface.
