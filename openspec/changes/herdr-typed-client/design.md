## Context

The workmux ↔ herdr integration exists upstream only as a CLI-driven backend
(`raine/workmux#195`, branch `GregoryTomy/workmux@herdr-backend`): every herdr
operation spawns `herdr` and parses a JSON envelope from stdout. Both tools are
Rust, both are bin-only (no library crate), so a library-level merge is not
available without forking, and a source merge is barred by the AGPL(herdr) /
MIT(workmux) license mismatch. herdr, however, ships a complete socket-API
JSON Schema (`herdr api schema --json`; `docs/next/api/herdr-api.schema.json`,
`protocol` 16, draft 2020-12) whose CLI is itself a thin wrapper — an explicit
"drive me programmatically" surface. A spike this session (recorded in
`mem:research/workmux-herdr-integration`, to be written on ship) fed the real
schema through `typify` and confirmed: all five sub-schemas generate with zero
errors/warnings (~280 types), compile clean (`serde` + `serde_json` +
`regress`), and round-trip to herdr's exact wire shape.

## Goals / Non-Goals

**Goals:**

- A typed, schema-generated Rust client (`herdr-client`) that the workmux fork
  consumes in place of CLI-shelling, with typed drift detection against herdr
  protocol changes.
- Deliver it to the fleet through pinned flake inputs with no Home Manager
  behavior change beyond the package source.

**Non-Goals:**

- The broader unification (worktree ownership, container/sandbox tier, status
  reconciliation, OpenSpec merge-gating) — separate track, see the proposal.
- The event/subscription stream (v2); any herdr-native worktree UI.
- Upstreaming as a blocker — done opportunistically, off this change's path.

## Decisions

- **Typed socket client over #195's CLI-shelling.** Chose generating request/
  response types and talking over the socket directly, rather than keeping the
  `herdr … | parse stdout` shape, because the CLI path spawns a process per
  call, is untyped, and breaks silently when herdr's protocol moves. The socket
  is the same surface herdr's own CLI uses.
- **Separate crate over merging the codebases.** Chose two binaries + a typed
  client at the socket boundary over fusing the trees, because herdr is
  AGPL-3.0 and workmux MIT — a merged work is virally AGPL — whereas a distinct
  program speaking a documented protocol is a clean boundary. It also keeps
  each tool on its own upstream cadence.
- **`typify` code-gen over hand-written types.** Chose `typify` because herdr's
  schema is schemars-derived draft-2020-12 and the spike proved a clean
  generate+compile+round-trip. Hand-maintaining ~280 types would drift from
  the source of truth immediately.
- **Checked-in generated types over `build.rs` codegen.** Chose committing the
  generated output with a regen script over generating at build time, because
  Nix builds must be hermetic/offline and a herdr protocol bump should surface
  as a *reviewable diff / compile break*, not a silent rebuild.
- **Per-sub-schema flattening over one merged root.** The schema is a container
  of five sub-schemas with **zero cross-references** but **12 same-named defs
  that differ** across them (e.g. `PaneInfo`, `EventData`). Chose generating
  each sub-schema independently (rewrite `#/schemas/<n>/$defs/X` → `#/$defs/X`)
  into its own module over merging into a global namespace, which would force
  ugly namespacing to avoid clobbering the differing defs.
- **Hand-written method veneer over the generated `Request` envelope enum.**
  `typify` renders the top-level `Request` `oneOf` as an untagged
  `Variant0..N` enum (const-discriminated on `method`) — unergonomic and lossy
  on deserialize. Chose consuming the params/result structs directly and
  hand-writing a ~40-line typed dispatch (`fn pane_split(PaneSplitParams) ->
  Result<…>` builds `{id, method:"pane.split", params}`), which is also the
  natural home for id-generation, socket send, and response correlation.
  Alternative (post-process the schema to name the `oneOf` branches) is more
  machinery for marginal gain — deferred.
- **Fork workmux + repoint the flake input over upstream-first.** `#195` is
  unmerged and the maintainer has not engaged; chose carrying a fork and
  upstreaming the *generic* parts opportunistically (the three worktree/
  workspace-lifecycle trait methods) over waiting on upstream.
- **Drop #88 candidate 3 (workmux-side wrapper detection).** herdr's own
  detector already resolves `.claude-wrapped` / `.opencode-wrapp` (leading dot,
  `-wrapped` suffix, macOS 15-char `comm` truncation) and ships a NixOS-path
  regression test, so once herdr is the multiplexer the detection gap is moot.

## Risks / Trade-offs

- **`typify` chokes on a future schema construct** → pin the schema per herdr
  release; the regen script + CI catch it; `typify` `--additional-*`/patch
  hooks are the escape valve. Spike showed the current schema needs none.
- **Fork rebase burden concentrates in workmux's four hottest workflow files**
  (`create/open/cleanup/setup.rs`, per the `#195` review) → keep herdr
  specifics isolated to `herdr.rs`; upstream the generic trait methods to
  shrink the delta.
- **Schema is a file in an AGPL repo** → generate fresh types *from* it rather
  than copying herdr's `src/api/schema.rs`; an API schema is an interface
  contract, and generated-from-spec is the defensible posture. The crate ships
  no herdr implementation source.
- **herdr protocol churn** (`protocol` 16 today) → pin + fail-loud on mismatch
  (spec requirement) + the drift diff.
- **No change on non-herdr hosts / darwin** → the backend only activates when
  `$HERDR_PANE_ID` is present; `herdr-client` is pure Rust and builds on
  darwin. Hosts on tmux are unaffected.

## Migration Plan

Repos touched and commit ordering (no k8s / terraform / ansible involvement;
the `home/` submodule needs **no** change — the package flows via the top-level
flake input's `follows` into `home-configs`):

1. **`herdr-client` (new repo, git.bph + GitHub mirror)** — schema-flatten
   transform, pinned schema, checked-in generated types, transport + typed
   veneer, CI regen-drift check. Commit and **push first** (a flake input needs
   a resolvable rev).
2. **`workmux` fork (git.bph + GitHub mirror)** — apply the `#195` branch,
   rewire `src/multiplexer/herdr.rs` onto `herdr-client`, fix the
   `$HERDR_BIN_PATH` deferred-cleanup bug. Depends on step 1; push second.
3. **nixspace `flake.nix`** — add the `herdr-client` input, repoint `workmux`
   from `github:raine/workmux` to the fork, `follows`-dedup. `git add`, then
   `nix run .#fmt` and `nix flake check --impure -j 1`; build workmux for `wsl`
   and `macbookpro`. Commit last.

**Rollback:** revert the `flake.nix` input repoint back to
`github:raine/workmux`. No data migration; the HM modules are untouched.

## Open Questions

- Crate module layout: one module per sub-schema vs a curated re-export of the
  request/response surface — decide at implementation.
- Whether to gate the herdr backend behind a cargo feature in the fork (so an
  eventual upstream PR can keep herdr's dep out of the default build).
- `herdr-client` version cadence vs herdr releases (pin-per-release vs track a
  protocol floor).
