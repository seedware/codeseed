# Codeseed

Codeseed is a command-line tool for managing agent skills inside a project.

Chinese version: [README.zh-CN.md](README.zh-CN.md).

The current project phase is product and architecture discovery. The initial brief is in [docs/project-brief.md](docs/project-brief.md).
The initial CLI surface is documented in [docs/cli.md](docs/cli.md).

## Planned Direction

Codeseed will start as a local CLI that can initialize agent skill directories, install preset skills, create compatibility symlinks for common agents, diagnose installed skills, and keep enough deterministic state to sync skill setup across Git checkouts.

The future SkillHub service is intentionally out of scope for the first milestone, but the CLI source model should be designed so SkillHub support can be added later without changing the core install flow.
