## ADDED Requirements

### Requirement: Typed socket client generated from herdr's published schema
The `herdr-client` crate SHALL provide Rust types for herdr's socket API
generated from herdr's own published JSON Schema (`herdr api schema --json`),
not hand-maintained. Generated types SHALL be committed to the crate (not
produced by build-time codegen) and reproduced by a regeneration script that
consumes a herdr-version-pinned copy of the schema, so that Nix builds are
hermetic and offline and any herdr protocol change appears as a reviewable
diff or a compile error.

#### Scenario: regeneration is reproducible
- **WHEN** the regeneration script is run against the pinned schema on a clean
  checkout
- **THEN** the committed generated types are reproduced with no diff

#### Scenario: a herdr protocol change is caught
- **WHEN** the pinned schema is bumped to a herdr release that changed a
  request or response shape
- **THEN** regeneration produces a non-empty diff and, where a field type
  changed, the crate or its consumer fails to compile — rather than
  mis-decoding at runtime

### Requirement: Protocol version is pinned and mismatches fail loud
The client SHALL record the herdr `protocol` version its types were generated
against and MUST surface a clear error when connected to a herdr server whose
protocol version is incompatible, rather than silently mis-parsing responses.

#### Scenario: incompatible server protocol
- **WHEN** the client connects to a herdr server reporting an incompatible
  protocol version
- **THEN** the client returns a typed error identifying the version mismatch

### Requirement: workmux herdr backend drives herdr through the typed client
The workmux herdr multiplexer backend SHALL perform all herdr control
operations by sending typed socket requests via `herdr-client`, not by
spawning the `herdr` CLI and parsing its stdout. The backend SHALL keep the
existing `Multiplexer` trait surface unchanged for its callers. Any deferred
shell script the backend emits MUST invoke herdr through `$HERDR_BIN_PATH`
when set, not a hardcoded `herdr`.

#### Scenario: a pane operation issues a typed request
- **WHEN** the backend splits, reads, or reports status on a herdr pane
- **THEN** it does so via a typed `herdr-client` call over the socket, with no
  `herdr` subprocess spawned for that operation

#### Scenario: deferred cleanup honors the configured binary
- **WHEN** the backend schedules deferred worktree/workspace cleanup on a host
  where `$HERDR_BIN_PATH` points at a non-`PATH` herdr binary
- **THEN** the deferred script invokes that binary, and cleanup succeeds

### Requirement: Fleet delivery via pinned flake inputs
The fleet's `workmux` package SHALL be built from the fork carrying the typed
herdr backend, sourced as a pinned flake input, with `herdr-client` provided as
a flake input the fork consumes and `follows`-deduplicated per the repo's
follows discipline. The `programs.workmux` and `programs.herdr` Home Manager
modules SHALL be unchanged except for the `workmux` package source.

#### Scenario: fresh host builds the herdr-backed workmux from pins
- **WHEN** a host that enables `programs.workmux` is built from the flake
- **THEN** the workmux binary it installs is the fork with the typed herdr
  backend, resolved entirely from pinned inputs with no network fetch of
  unpinned sources

#### Scenario: no Home Manager behavior change
- **WHEN** the workmux input is repointed to the fork
- **THEN** the generated workmux/herdr user configuration is unchanged apart
  from the package derivation

### Requirement: License boundary preserved
The `herdr-client` crate SHALL be independently licensed (MIT or Apache-2.0)
and SHALL contain only types generated from herdr's schema plus a
hand-written transport; it MUST NOT vendor herdr's implementation source. The
crate communicates with herdr solely over herdr's documented socket protocol.

#### Scenario: crate carries no AGPL implementation
- **WHEN** the crate's sources are inspected
- **THEN** they consist of schema-generated types and a transport/veneer under
  a permissive license, with no copied herdr implementation files
