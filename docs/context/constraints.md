# Constraints Context

Follow these constraints when changing Codeseed.

## Documentation

1. Every user-facing English Markdown document should have a Chinese counterpart when practical.
2. Keep `docs/context/README.md` concise. Put details in focused context files.
3. Update docs when command behavior changes.

## Rust

1. Prefer existing modules and helpers before adding abstractions.
2. Keep CLI parsing in `src/cli.rs`.
3. Keep filesystem behavior deterministic and safe around user-owned files.
4. Avoid destructive filesystem operations unless the command explicitly owns the path or the user requested force behavior.
5. Use relative symlink targets for committed project compatibility links.

## Verification

Run these after Rust changes:

```bash
cargo fmt --check
cargo test
```

For CLI behavior changes, also run the relevant help or command output, for example:

```bash
cargo run -- list
cargo run -- update --dry-run
cargo run -- init
```

## Generated Skill Content

When changing preset skills:

1. update `presets/skills/<skill-id>/`;
2. install or refresh the skill into `.agent/skills/common/<skill-id>/` when this repository should use it;
3. verify `.codeseed/state.json` and compatibility links.

