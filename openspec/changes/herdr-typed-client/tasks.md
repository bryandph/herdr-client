## 1. `herdr-client` crate: schema + generation

- [x] 1.1 Create the `herdr-client` repo (git.bph `bryan/herdr-client`, done; GitHub mirror later), tracking issue `bryan/herdr-client#1`, openspec moved in. Scaffold the crate: dual `LICENSE-MIT` + `LICENSE-APACHE`, `Cargo.toml` (`license = "MIT OR Apache-2.0"`), and pin `herdr-api.schema.json` recording the herdr version + `protocol` number it was captured at
- [x] 1.2 `scripts/flatten.py` (rewrites `#/schemas/<n>/$defs/X` → `#/$defs/X` per sub-schema; errors on any cross-sub-schema ref) + `scripts/regen.sh` (flatten → `cargo-typify -d PartialEq` → `rustfmt`) into committed `src/generated/<schema>.rs`. Flatten output verified JSON-identical to the spike roots typify already consumed
- [x] 1.3 Generated types committed to `src/generated/`; `cargo check` + `cargo clippy` clean under the flake devShell (rust 1.96.2, `cargo-typify` 0.6.2; deps `serde`/`serde_json`/`regress`)
- [x] 1.4 `.woodpecker/ci.yaml` re-runs `regen.sh` + `git diff --exit-code src/generated` (drift gate) alongside fmt/clippy/test. Determinism verified locally (regen twice → zero diff). Runner must be enabled for this repo in the Woodpecker server (infra follow-up)

## 2. `herdr-client` crate: transport + typed veneer

- [x] 2.1 `src/transport.rs`: `Connection::{from_env,connect}` over `$HERDR_SOCKET_PATH`, newline-delimited JSON frames, `req_N` id correlation (skips interleaved non-matching lines), typed `Error` in `src/error.rs`
- [x] 2.2 `PROTOCOL` const (generated into `src/generated/protocol.rs` from the pinned schema); `Connection::check_protocol` pings and returns `Error::ProtocolMismatch` against an incompatible server
- [x] 2.3 `src/methods.rs`: typed method fns over the params structs for the workmux surface (pane split/get/current/read/list/send_text/send_keys/send_input/close/rename/zoom/report_agent, workspace create/get/list/close/rename, tab create/get/list/close, worktree create/open/remove/list, events.wait). Params fully typed; results pinned to concrete types where herdr's schema names them unambiguously (`*Info` for `get`, dedicated `*Result`), `serde_json::Value` + the generic `call<P,R>` escape hatch elsewhere — never the untagged `Request` enum. (Result-type tightening for create/list pending confirmation of herdr's method→result map.)
- [x] 2.4 `tests/wire.rs`: `pane.split` frame round-trip (exact wire shape) + `pong` decode; untagged-envelope rationale documented in `README.md`

## 3. workmux fork: rewire the herdr backend

- [x] 3.1 Fork base ready locally (`bryandph/workmux` branch `herdr-typed-client`): cherry-picked `GregoryTomy@herdr-backend` onto current `raine/workmux` main (v0.1.222) — clean, builds. **GitHub fork push pending owner auth** (MCP token lacks fork scope, `gh` token expired)
- [x] 3.2 Added the `herdr-client` path dependency and rewired `src/multiplexer/herdr.rs`: `run()`/`run_json()` CLI spawns → per-op `herdr_client::Connection` (from `$HERDR_SOCKET_PATH`) with typed `request::*` params. Result-plucking unchanged (socket `result` == CLI `result`); `Multiplexer` trait surface identical; `bin` kept only for deferred/shell-hook commands
- [x] 3.3 Fixed the deferred-cleanup bug in `src/workflow/cleanup.rs`: `${HERDR_BIN_PATH:-herdr}` shell expansion instead of the hardcoded `herdr`; test updated
- [x] 3.4 `cargo check` + `cargo clippy` clean. **Live-verified on macbookpro** against herdr 0.7.3 (built + switched via nixspace): backend detection selects `herdr`; live protocol 16 == pinned; `ping` wire shape matches the transport; `workmux status` (socket list) clean; `workmux add --mode window` issued a typed `worktree.create` that created a real herdr workspace + git worktree; `workspace.list` read it back; session-mode correctly rejected. (`cargo test` binary still won't link under the local nix-darwin cctools linker — env bug — so the unit test-run belongs to Linux CI.)

## 4. nixspace integration (verification gate)

- [x] 4.1 Repointed the `workmux` input `github:raine/workmux` → `github:bryandph/workmux/herdr-typed-client` (`flake.nix`), `nix flake update workmux` locked it (rev a6c3c0ef). **No separate `herdr-client` flake input needed** — the fork's `buildRustPackage` vendors herdr-client via `cargoLock.outputHashes` (simpler than the design assumed; `follows`-dedup N/A since it's a cargo git dep, not a flake input)
- [x] 4.2 `home/` submodule needs no edit: workmux flows via `workmux.follows` into `home-configs` (nixspace `flake.nix`), so only the input url changed; the `programs.workmux`/`programs.herdr` modules are untouched
- [~] 4.3 `nix run --impure .#fmt` clean. Fork package derivation evals from pins for **both** `x86_64-linux` and `aarch64-darwin`. **macbookpro built + switched the fork cleanly** (the packaged `buildRustPackage` link works on darwin — the cctools trap only hit the local `cargo test` binary). Remaining: `nix flake check --impure -j 1` + the `wsl` host closure (CI). nixspace `flake.nix`/`flake.lock` change staged, not committed (parent repo on `main`; awaiting review)

## 5. Upstreaming + knowledge capture

- [ ] 5.1 Open the standalone upstream PR for #88 candidate 2 (Codex `PermissionRequest` → waiting); offer the three generic `Multiplexer` worktree/workspace trait methods to `raine/workmux`; offer `herdr-client` to herdr as an official Rust client
- [x] 5.2 Knowledge capture: `mem:research/workmux-herdr-integration` written (spike findings + layered architecture + open unification decisions); Gitea `nixspace#88` restructured (candidate 1 → typed-client, candidate 3 dropped)
- [ ] 5.3 On ship: close `bryan/herdr-client#1`, archive this change (`openspec archive herdr-typed-client`)
