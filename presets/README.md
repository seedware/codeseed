# Codeseed Presets

Chinese version: [README.zh-CN.md](README.zh-CN.md).

This directory contains built-in skills shipped with Codeseed.

Preset skills are used for two purposes:

1. `codeseed init` can install them automatically into a target project.
2. Codeseed development can exercise `add`, `rm`, `doctor`, and `sync` behavior without depending on an external SkillHub.

## Layout

```text
presets/
└── skills/
    └── <skill-id>/
        ├── skill.toml
        └── SKILL.md
```

## Source Syntax

Preset skills should be addressable through the normal `add` flow with an explicit source prefix:

```text
codeseed add preset:<skill-id>
```

For example:

```text
codeseed add preset:codeseed-skill-author
codeseed add preset:codeseed-context-index
codeseed add preset:codeseed-multi-git-remote
codeseed add preset:codeseed-prebuilt-release
```

## Manifest

Each preset skill must include `skill.toml`. The first manifest version is intentionally small:

```toml
schema = 1
id = "codeseed-skill-author"
name = "Codeseed Skill Author"
version = "0.1.0"
target = "common"
entry = "SKILL.md"
```

`target` uses the same target names as the CLI: `common`, `codex`, `claude`, or `cursor`.
