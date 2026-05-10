# Codeseed Skill Author

Use this skill when creating, reviewing, or improving agent skills for a project managed by Codeseed.

## Workflow

1. Identify the target agent or agents for the skill.
2. Keep the skill focused on one repeatable capability.
3. Define the expected files, entry document, and placement target.
4. Include a Chinese version for every user-facing Markdown document.
5. Prefer small examples that can be validated by `codeseed doctor`.
6. Avoid hidden dependencies on a remote SkillHub unless the skill explicitly documents them.

## Output Expectations

When producing a skill, include:

1. `skill.toml`
2. `SKILL.md`
3. `SKILL.zh-CN.md`
4. Any referenced assets or scripts

The skill should be installable from a local directory before it is published anywhere else.

