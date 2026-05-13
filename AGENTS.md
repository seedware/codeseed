# Codeseed Agent Instructions

This repository is managed by Codeseed for project-local agent skills.

## Skills

- Read `docs/context/README.md` first when starting a new thread or when project background is unclear.
- Canonical skills live under `.agent/skills/`.
- Codeseed metadata lives under `.codeseed/`.
- Discover installed skills by scanning `.agent/skills/common/*/skill.toml` and each skill's `SKILL.md` front matter.
- If a skill's `SKILL.md` front matter has `alwaysApply: true`, read that skill's `skill.toml` and full `SKILL.md` at the start of every task.
- When a task matches a skill's `name`, `description`, `triggers`, or `default_behavior`, read that skill's `skill.toml` and full `SKILL.md` before acting.
- Do not enumerate individual skills here. Skill-specific trigger rules and default behavior belong in the skill's own `SKILL.md` front matter.
- Before changing skill files, inspect the matching `skill.toml` and `SKILL.md`.
- Codeseed-managed skills use a single Chinese `SKILL.md`; do not add parallel localized skill documents.

## Verification

- Run `cargo fmt --check` after Rust edits.
- Run `cargo test` after CLI or skill-management changes.
