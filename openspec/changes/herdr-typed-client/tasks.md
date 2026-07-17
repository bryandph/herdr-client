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
- [~] 3.4 `cargo check` + `cargo clippy` clean (rust 1.96, herdr-client path dep). Test-run (deferred-cleanup/backend-detection) + live `workmux add` → herdr workspace **deferred to the Linux Woodpecker runner**: the nix-darwin `cctools` linker traps (exit 133) linking the large workmux test binary locally — an environment bug, not a code defect

## 4. nixspace integration (verification gate)

- [ ] 4.1 Add the `herdr-client` flake input and repoint the `workmux` input from `github:raine/workmux` (`flake.nix:400`) to the fork; `follows`-dedup per `mem:style/nix-follows-discipline`; `git add` the new files
- [ ] 4.2 Confirm the `home/` submodule needs no edit (package flows via the input's `follows` into `home-configs`); confirm the generated workmux/herdr user config is unchanged apart from the derivation
- [ ] 4.3 Verification gate: `nix run --impure .#fmt` then `nix flake check --impure -j 1`; build `nixosConfigurations.wsl` and `darwinConfigurations.macbookpro` workmux closures to confirm the fork + crate resolve from pins

## 5. Upstreaming + knowledge capture

- [ ] 5.1 Open the standalone upstream PR for #88 candidate 2 (Codex `PermissionRequest` → waiting); offer the three generic `Multiplexer` worktree/workspace trait methods to `raine/workmux`; offer `herdr-client` to herdr as an official Rust client
- [x] 5.2 Knowledge capture: `mem:research/workmux-herdr-integration` written (spike findings + layered architecture + open unification decisions); Gitea `nixspace#88` restructured (candidate 1 → typed-client, candidate 3 dropped)
- [ ] 5.3 On ship: close `bryan/herdr-client#1`, archive this change (`openspec archive herdr-typed-client`)
