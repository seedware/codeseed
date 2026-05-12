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
codeseed init [--agent-dir <DIR>] [--codeseed-dir <DIR>] [--no-presets] [--no-links] [--language <LANGUAGE>] [--force]
```

Options:

1. `--agent-dir <DIR>`: agent-facing directory to create and manage. Defaults to `.agent`.
2. `--codeseed-dir <DIR>`: Codeseed metadata directory. Defaults to `.codeseed`.
3. `--no-presets`: skip bundled preset skills.
4. `--no-links`: skip compatibility links for known agents.
5. `--language <LANGUAGE>`: language for generated instructions and installed preset skill documents. Defaults to `en`.
6. `--force`, `-f`: overwrite incompatible generated Codeseed state when possible.

Languages:

1. `en`
2. `zh-CN`

Codeseed installs only one instruction language into generated project files. For Chinese preset skills, the Chinese document becomes the installed `SKILL.md` entry instead of installing both English and Chinese entry documents.

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

### `codeseed list`

Lists available built-in skills or installed project skills.

```text
codeseed list [--installed] [--format <FORMAT>]
```

Options:

1. `--installed`: list skills installed in the current project instead of built-in available skills.
2. `--format <FORMAT>`: output format. Defaults to `text`.

Formats:

1. `text`
2. `json`

### `codeseed update`

Updates the `codeseed` executable itself.

```text
codeseed update [--version <VERSION>] [--home <DIR>] [--bin-dir <DIR>] [--mode <MODE>] [--script-url <URL>] [--dry-run]
```

Behavior:

1. If a local `scripts/install.sh` exists, Codeseed uses it.
2. Otherwise Codeseed downloads the install script and runs it with `sh`.
3. The installer installs into `~/.codeseed/bin` by default.

Options:

1. `--version <VERSION>`: Codeseed version to install. Defaults to `latest`.
2. `--home <DIR>`: Codeseed home directory. Defaults to `~/.codeseed` in the installer.
3. `--bin-dir <DIR>`: directory for the executable. Defaults to `~/.codeseed/bin` in the installer.
4. `--mode <MODE>`: installer strategy. Defaults to `auto`.
5. `--script-url <URL>`: install script URL used when no local installer is available.
6. `--dry-run`: show the update plan without executing it.

Modes:

1. `auto`
2. `local`
3. `prebuilt`

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

### `codeseed clear`

Clears Codeseed-managed state and generated agent content from a project.

```text
codeseed clear [--agent-dir <DIR>] [--codeseed-dir <DIR>] [--dry-run] [--yes --confirm clear-codeseed-state]
```

This is a destructive command. By default, Codeseed must ask for an interactive second confirmation before removing anything.

Options:

1. `--agent-dir <DIR>`: agent-facing directory to remove. Defaults to `.agent`.
2. `--codeseed-dir <DIR>`: Codeseed metadata directory to remove. Defaults to `.codeseed`.
3. `--dry-run`: show what would be removed without modifying files.
4. `--yes`: skip the interactive confirmation prompt.
5. `--confirm clear-codeseed-state`: required confirmation phrase when using `--yes`.

`--yes` and `--confirm clear-codeseed-state` must be used together. They cannot be combined with `--dry-run`.
