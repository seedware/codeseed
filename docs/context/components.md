# Components Context

This file lists reusable project pieces and where to look before adding more.

## CLI Types

Use `src/cli.rs` for all command argument types:

1. command enum: `Command`;
2. command structs: `InitCommand`, `AddCommand`, `ListCommand`, `UpdateCommand`, and others;
3. shared enums: `OutputFormat`, `SkillTarget`, `UpdateMode`.

## Error Handling

Use `CodeseedError` from `src/error.rs`.

Current variants:

1. `Io`: filesystem or process IO failures with a path/context;
2. `Conflict`: user-facing conflict or unsupported state.

## Presets

Preset metadata lives in `src/presets.rs`.

Preset content lives in `presets/skills/<skill-id>/`.

Current preset skills:

1. `codeseed-skill-author`
2. `codeseed-context-index`
3. `codeseed-multi-git-remote`

## Generated Compatibility

Compatibility generation currently lives in `src/init.rs`:

1. Claude links: `.claude/skills/<skill-id>`;
2. Cursor rules: `.cursor/rules/<skill-id>.mdc`;
3. generic agent guidance: `AGENTS.md`.

Prefer reusing these helpers before adding separate generation code.

