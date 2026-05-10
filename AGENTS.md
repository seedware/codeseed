# Codeseed Agent Instructions

This repository is managed by Codeseed for project-local agent skills.

## Skills

- Canonical skills live under `.agent/skills/`.
- Codeseed metadata lives under `.codeseed/`.
- Before changing skill files, inspect the matching `skill.toml` and `SKILL.md`.
- Every user-facing Markdown skill document should have a Chinese version when practical.

## Verification

- Run `cargo fmt --check` after Rust edits.
- Run `cargo test` after CLI or skill-management changes.
