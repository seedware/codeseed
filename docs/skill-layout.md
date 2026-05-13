# Skill Layout

Chinese version: [skill-layout.zh-CN.md](skill-layout.zh-CN.md).

Codeseed uses one canonical project-local skill layout, then generates compatibility entries for specific agent tools.

## Existing Conventions

The current ecosystem does not have one universal project directory used by every agent, but there is a strong common shape:

1. Agent Skills compatible tools treat a skill as a directory with a required `SKILL.md` file and optional `scripts/`, `references/`, and `assets/` directories.
2. Claude Code project skills live under `.claude/skills/<skill-name>/`.
3. Cursor project rules live under `.cursor/rules/*.mdc`; Cursor also supports root-level `AGENTS.md` for simple project instructions.
4. Codex and many coding agents understand `AGENTS.md` as a project instruction file.

## Codeseed Canonical Layout

```text
.
├── .agent/
│   ├── skills/
│   │   ├── common/
│   │   │   └── <skill-id>/
│   │   │       ├── skill.toml
│   │   │       └── SKILL.md
│   │   ├── codex/
│   │   ├── claude/
│   │   └── cursor/
│   └── generated/
│       └── cursor-rules/
├── .codeseed/
│   └── state.json
├── .claude/
│   └── skills/
├── .cursor/
│   └── rules/
└── AGENTS.md
```

Rules:

1. `.agent/skills/` is the canonical Codeseed-owned skill tree.
2. `.agent/skills/common/` contains skills that can be exposed to multiple agents.
3. Agent-specific directories are reserved for skills or adapters that only make sense for one agent.
4. `.agent/generated/` contains generated compatibility artifacts.
5. `.codeseed/` records desired state for future `sync`.
6. Tool-native directories such as `.claude/skills/` and `.cursor/rules/` should be generated from Codeseed state.

## Compatibility Strategy

Claude Code:

```text
.claude/skills/<skill-id> -> .agent/skills/common/<skill-id>
```

Cursor:

```text
.cursor/rules/<skill-id>.mdc -> .agent/generated/cursor-rules/<skill-id>.mdc
```

Codex and generic coding agents:

```text
AGENTS.md
```

`AGENTS.md` points agents to the Codeseed-managed skill tree and records repository-level operating guidance.

## Skill Package Shape

Each skill directory should be compatible with the Agent Skills shape:

```text
<skill-id>/
├── SKILL.md
├── skill.toml
├── scripts/
├── references/
└── assets/
```

Only `SKILL.md` is required by the broader Agent Skills convention. Codeseed additionally uses `skill.toml` for local management metadata. Codeseed-managed skills keep a single `SKILL.md`; current preset skills use Chinese content in that entry file instead of parallel localized skill documents.

## Skill Activation

Agents should discover skills through a generic entrypoint rather than hard-coded skill names in `AGENTS.md`.

Each `SKILL.md` should declare activation metadata in its front matter:

```yaml
---
name: example-skill
description: Short natural-language summary of when to use this skill.
triggers:
  - user phrase or task family
  - another matching cue
default_behavior:
  - Default action or interpretation when the trigger is ambiguous.
alwaysApply: false
---
```

Rules:

1. `AGENTS.md` should tell agents to scan installed skills, always load skills with `alwaysApply: true`, and match against `name`, `description`, `triggers`, and `default_behavior`.
2. `AGENTS.md` should not enumerate individual skills or their task-specific behavior.
3. Skill-specific routing and default behavior belong in the skill's own front matter and full `SKILL.md` body.
4. Generated compatibility entries should point back to the canonical skill instead of becoming the source of truth.
5. `alwaysApply: true` is optional and is copied into generated Cursor rules for skills that must load on every request.

## Refreshing Project Skills

Project skills are stored inside the project, so updating the global `codeseed` executable does not automatically change an initialized project. Refresh the project from its root directory.

For the currently implemented preset source, reinstall the preset with `--force`:

```bash
codeseed add preset:<skill-id> --force
```

For example:

```bash
codeseed add preset:codeseed-context-index --force
```

After refreshing, verify the installed skill list and generated compatibility entries:

```bash
codeseed list --installed
ls -l .claude/skills .cursor/rules
```

For Cursor rules, each `.cursor/rules/<skill-id>.mdc` entry should be a symlink to `.agent/generated/cursor-rules/<skill-id>.mdc`. For Claude Code, each `.claude/skills/<skill-id>` entry should point at `.agent/skills/common/<skill-id>`.

If the command is available in the installed version, `codeseed doctor --fix` can repair safe generated-directory or compatibility-link issues, and `codeseed sync --check` can check recorded generated state without rewriting files.
