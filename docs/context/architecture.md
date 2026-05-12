# Architecture Context

Codeseed is a Rust CLI for managing project-local agent skills.

## Product Shape

Codeseed currently focuses on local project operations:

1. Initialize a project-local skill layout.
2. Install built-in preset skills.
3. Generate compatibility entries for common agents.
4. Track desired skill state in `.codeseed/state.json`.
5. Provide CLI commands for list, add, update, and later rm/doctor/sync/clear behavior.

The future SkillHub is intentionally out of scope for the current repository, but command and source resolution design should leave room for it.

## Canonical Project Layout

The canonical skill tree is `.agent/skills/`.

Current generated project layout:

```text
.agent/
  skills/common/<skill-id>/
  generated/cursor-rules/<skill-id>.mdc
.codeseed/state.json
.claude/skills/<skill-id> -> ../../.agent/skills/common/<skill-id>
.cursor/rules/<skill-id>.mdc -> ../../.agent/generated/cursor-rules/<skill-id>.mdc
AGENTS.md
docs/context/
```

Tool-native directories are generated compatibility entries. `.agent/skills/common/` is the canonical source for installed skill content.

## State Model

`.codeseed/state.json` currently records:

1. schema version;
2. agent directory;
3. Codeseed metadata directory;
4. installed skill ids, sources, and targets.

Keep this file deterministic and reviewable. It is intended to be committed.

## Install And Update

Installation uses [scripts/install.sh](../../scripts/install.sh). It installs into `~/.codeseed/bin` by default and creates `~/.codeseed/config` and `~/.codeseed/cache`.

`codeseed update` reuses the install script:

1. use local `scripts/install.sh` when available;
2. otherwise download the published script;
3. support `--dry-run` so users can inspect the plan.

