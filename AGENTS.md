# Codeseed Agent Instructions

This repository is managed by Codeseed for project-local agent skills.

## Skills

- Read `docs/context/README.md` first when starting a new thread or when project background is unclear.
- Canonical skills live under `.agent/skills/`.
- Codeseed metadata lives under `.codeseed/`.
- When a task matches an installed skill, read that skill's `skill.toml` and `SKILL.md` before acting.
- For Git remote, branch, commit, push, pull, or fetch work, read `docs/context/git.md` and use `.agent/skills/common/codeseed-multi-git-remote/SKILL.md` when present.
- Before changing skill files, inspect the matching `skill.toml` and `SKILL.md`.
- Every user-facing Markdown skill document should have a Chinese version when practical.

## Verification

- Run `cargo fmt --check` after Rust edits.
- Run `cargo test` after CLI or skill-management changes.
