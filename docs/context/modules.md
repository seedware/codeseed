# Module Context

Rust source lives under `src/`.

## Module Map

1. `src/cli.rs`: clap command and argument definitions. Add command shape here first.
2. `src/main.rs`: top-level command dispatch and user-facing command output.
3. `src/error.rs`: shared `CodeseedError` and `Result` type.
4. `src/init.rs`: `codeseed init`, generated layout, preset installation helpers, compatibility links, context skeleton.
5. `src/add.rs`: currently implements `add preset:<skill-id>` and updates state plus compatibility entries.
6. `src/list.rs`: lists built-in preset skills or installed project skills.
7. `src/update.rs`: updates the Codeseed executable by running the install script.
8. `src/presets.rs`: constants for preset skill ids and preset source prefix.
9. `src/lib.rs`: module exports.

## Command Implementation Pattern

For new commands:

1. Define args in `src/cli.rs`.
2. Add a module with `run(project, command) -> Result<Report>`.
3. Wire dispatch in `src/main.rs`.
4. Add focused unit tests in the command module and clap parsing tests in `src/cli.rs`.
5. Update [docs/cli.md](../cli.md) and [docs/cli.zh-CN.md](../cli.zh-CN.md).

## Current Gaps

The command surface exists for `rm`, `doctor`, `sync`, and `clear`, but they are not fully implemented yet.

