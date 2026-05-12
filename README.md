# Codeseed

Codeseed is a command-line tool for managing agent skills inside a project.

Chinese version: [README.zh-CN.md](README.zh-CN.md).

The current project phase is product and architecture discovery. The initial brief is in [docs/project-brief.md](docs/project-brief.md).
The initial CLI surface is documented in [docs/cli.md](docs/cli.md).
Installation is documented in [docs/install.md](docs/install.md).
The project-local skill layout is documented in [docs/skill-layout.md](docs/skill-layout.md).
Built-in preset skills live in [presets/](presets/).

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh | sh
```

## Update

Update the `codeseed` executable:

```bash
codeseed update --dry-run
codeseed update
```

Refresh an installed preset skill in a project:

```bash
codeseed add preset:<skill-id> --force
```

Then verify the project-local generated entries:

```bash
codeseed list --installed
ls -l .claude/skills .cursor/rules
```

More detail is in [docs/install.md](docs/install.md) for updating Codeseed itself and [docs/skill-layout.md](docs/skill-layout.md) for refreshing project skills and compatibility links.

## Planned Direction

Codeseed will start as a local CLI that can initialize agent skill directories, install preset skills, create compatibility symlinks for common agents, diagnose installed skills, and keep enough deterministic state to sync skill setup across Git checkouts.

The future SkillHub service is intentionally out of scope for the first milestone, but the CLI source model should be designed so SkillHub support can be added later without changing the core install flow.
