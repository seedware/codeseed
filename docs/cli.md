# Codeseed CLI

Chinese version: [cli.zh-CN.md](cli.zh-CN.md).

This document describes the initial Codeseed command-line surface. The implementation is currently a Rust CLI using `clap`.

## Global Options

```text
codeseed --project <DIR> <COMMAND>
```

Options:

1. `--project <DIR>`: project root to operate on. Defaults to the current directory.

## Commands

### `codeseed init`

Initializes Codeseed in a project.

```text
codeseed init [--agent-dir <DIR>] [--codeseed-dir <DIR>] [--no-presets] [--no-links] [--force]
```

Options:

1. `--agent-dir <DIR>`: agent-facing directory to create and manage. Defaults to `.agent`.
2. `--codeseed-dir <DIR>`: Codeseed metadata directory. Defaults to `.codeseed`.
3. `--no-presets`: skip bundled preset skills.
4. `--no-links`: skip compatibility links for known agents.
5. `--force`, `-f`: overwrite incompatible generated Codeseed state when possible.

### `codeseed add <SOURCE>`

Adds a skill from Codeseed presets, SkillHub, GitHub, URL, file, or directory.

```text
codeseed add <SOURCE> [--hub <URL>] [--name <NAME>] [--target <TARGET>] [--target-dir <DIR>] [--force]
```

Sources can include `preset:<skill-id>`, a SkillHub id, GitHub reference, URL, local file, or local directory.

Options:

1. `--hub <URL>`: resolve the source from a specific SkillHub endpoint.
2. `--name <NAME>`: install the skill under a specific local name.
3. `--target <TARGET>`: override placement inferred from the skill manifest.
4. `--target-dir <DIR>`: override destination directory.
5. `--force`, `-f`: replace an existing managed skill with the same id or name.

Targets:

1. `common`
2. `codex`
3. `claude`
4. `cursor`

### `codeseed rm <SKILL>`

Removes an installed Codeseed-managed skill.

```text
codeseed rm <SKILL> [--force] [--prune]
```

Aliases:

1. `codeseed remove <SKILL>`

Options:

1. `--force`, `-f`: remove generated files even when local state is partially inconsistent.
2. `--prune`: remove empty generated parent directories after the skill is removed.

### `codeseed doctor`

Diagnoses Codeseed state, skill manifests, and compatibility links.

```text
codeseed doctor [--strict] [--fix] [--format <FORMAT>]
```

Options:

1. `--strict`: treat warnings as failures.
2. `--fix`: attempt safe repairs for generated directories and compatibility links.
3. `--format <FORMAT>`: output format. Defaults to `text`.

Formats:

1. `text`
2. `json`

### `codeseed sync`

Reconciles generated files from recorded Codeseed state.

```text
codeseed sync [--dry-run] [--check] [--prune]
```

Options:

1. `--dry-run`: show planned changes without modifying files.
2. `--check`: check whether generated files match recorded state, without repairing them.
3. `--prune`: remove stale generated files when they are still owned by Codeseed.

`--dry-run` and `--check` are mutually exclusive.
