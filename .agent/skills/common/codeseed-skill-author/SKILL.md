---
name: codeseed-skill-author
description: Create, review, and improve Codeseed-managed agent skills. Use when working on skill.toml, SKILL.md, preset skills, or project skill metadata.
license: MIT
compatibility: Designed for Codeseed-managed project skills and Agent Skills compatible clients.
triggers:
  - skill.toml
  - SKILL.md
  - preset skill
  - project skill metadata
  - skill authoring
default_behavior:
  - Put skill-specific trigger rules and default behavior in the skill's own SKILL.md front matter.
  - Do not require AGENTS.md to enumerate individual skills.
metadata:
  codeseed.version: "0.1.0"
---

# Codeseed Skill Author

Use this skill when creating, reviewing, or improving agent skills for a project managed by Codeseed.

## Workflow

1. Identify the target agent or agents for the skill.
2. Keep the skill focused on one repeatable capability.
3. Define the expected files, entry document, and placement target.
4. Put activation cues in `SKILL.md` front matter, especially `description`, `triggers`, and `default_behavior`.
5. Keep `AGENTS.md` as a generic skill discovery entrypoint; do not hard-code individual skill names there.
6. Include a Chinese version for every user-facing Markdown document.
7. Prefer small examples that can be validated by `codeseed doctor`.
8. Avoid hidden dependencies on a remote SkillHub unless the skill explicitly documents them.

## Output Expectations

When producing a skill, include:

1. `skill.toml`
2. `SKILL.md`
3. `SKILL.zh-CN.md`
4. Any referenced assets or scripts

The skill should be installable from a local directory before it is published anywhere else.
