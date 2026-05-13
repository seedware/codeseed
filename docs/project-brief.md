# Codeseed Project Brief

Chinese version: [project-brief.zh-CN.md](project-brief.zh-CN.md).

## Positioning

Codeseed is a command-line tool for managing agent skills inside a target project.

In its current form, Codeseed should focus on local project initialization and skill lifecycle management. The long-term direction is to become the local companion for a future SkillHub ecosystem, while staying useful without any remote service.

## Goals

1. `codeseed init` initializes the current directory for agent skill management.
2. Initialization creates a `.agent` directory with the required skill directory structure.
3. Initialization installs a small set of built-in preset skills from the repository `presets/skills/` directory.
4. Initialization creates compatibility symlinks for common agents, including Claude Code, Codex, Cursor, and similar tools.
5. `codeseed add` and `codeseed rm` manage skills after initialization.
6. Skill placement is driven by skill configuration, so Codeseed can put each skill in the correct target directory.
7. Skill sources should support:
   - the default SkillHub;
   - a user-specified SkillHub;
   - a single skill file from a URL;
   - a single local skill file;
   - a local skill directory;
   - GitHub repositories or paths.
8. `codeseed doctor` validates all installed skills and reports invalid or incompatible skills.
9. Codeseed should work well with Git.
10. Codeseed should keep enough local operation state to support `codeseed sync` across different local checkouts of the same project.

## Non-Goals For The First Milestone

1. Building SkillHub itself.
2. Implementing remote publishing, rating, search, or account features.
3. Supporting every possible agent-specific skill format on day one.
4. Hiding the generated files from users. The generated `.agent` state should be understandable and reviewable.

## Initial Command Surface

### `codeseed init`

Creates the local Codeseed-managed agent skill structure.

Expected behavior:

1. Create `.agent/`.
2. Create necessary skill directories.
3. Install bundled preset skills.
4. Create compatibility symlinks for agent tools.
5. Create `.codeseed/` metadata for local state and future sync support.
6. Refuse to overwrite incompatible existing state unless the user passes an explicit force option.

### `codeseed add <source>`

Adds a skill from a supported source.

Expected source types:

1. Default SkillHub identifier, for example `codeseed add test-writer`.
2. Explicit SkillHub identifier, for example `codeseed add --hub <url> test-writer`.
3. Remote skill file URL.
4. Local skill file path.
5. Local skill directory path.
6. GitHub repository or repository subpath.

Expected behavior:

1. Resolve the source.
2. Read skill configuration.
3. Validate the skill.
4. Place files in the correct managed directory.
5. Record the operation in `.codeseed/`.

### `codeseed list`

Lists skills known to Codeseed.

Expected behavior:

1. List built-in preset skills by default.
2. Support listing installed project skills.
3. Support machine-readable output for scripts.
4. Later extend to SkillHub-backed sources without changing the command shape.

### `codeseed update`

Updates the Codeseed executable itself.

Expected behavior:

1. Use the local install script when running from a Codeseed source checkout.
2. Otherwise download and run the published install script.
3. Support selecting a version.
4. Support dry-run output so users can inspect the update plan first.
5. Reuse the same `~/.codeseed` home directory model as installation.

### `codeseed rm <skill>`

Removes a managed skill.

Expected behavior:

1. Find the installed skill by name or id.
2. Remove managed files.
3. Preserve user-owned files unless they are explicitly managed by Codeseed.
4. Record the operation in `.codeseed/`.

### `codeseed doctor`

Validates the current Codeseed and skill state.

Expected checks:

1. `.agent/` exists and matches the expected structure.
2. Compatibility symlinks exist and point to the expected locations.
3. Installed skill manifests are valid.
4. Skill files referenced by manifests exist.
5. Agent-specific output directories are consistent.
6. `.codeseed/` state is readable and internally consistent.

### `codeseed sync`

Synchronizes Codeseed-managed local state across different local directories of the same Git project.

Expected behavior:

1. Read `.codeseed/` state.
2. Compare desired installed skill state with current `.agent/` contents.
3. Recreate missing generated files and symlinks.
4. Remove stale generated files when safe.
5. Keep behavior deterministic so Git can track meaningful changes.

### `codeseed clear`

Clears Codeseed-managed state and generated agent content from the target project.

Expected behavior:

1. Treat the command as destructive.
2. Support `--dry-run` so users can inspect the removal plan first.
3. Require an interactive second confirmation before deleting files by default.
4. Require both `--yes` and `--confirm clear-codeseed-state` for non-interactive execution.
5. Remove Codeseed-owned metadata and generated agent-facing content only.
6. Avoid deleting user-owned files that are not recorded as Codeseed-managed.

## Proposed Directory Model

```text
.
├── .agent/
│   ├── skills/
│   │   ├── common/
│   │   ├── codex/
│   │   ├── claude/
│   │   └── cursor/
│   └── links/
├── .codeseed/
│   ├── state.json
│   ├── lock.json
│   └── operations/
└── docs/
    └── project-brief.md
```

The exact directory names are still open for discussion. The important distinction is:

1. `.agent/` contains the agent-facing skill files and compatibility layout.
2. `.codeseed/` contains Codeseed-owned metadata that explains what was installed, from where, and how to reproduce it.

## Compatibility Symlink Intent

Codeseed should expose the same underlying managed skill content to multiple agent tools without forcing users to duplicate files manually.

Possible compatibility targets:

1. Claude Code skill directory.
2. Codex skill directory.
3. Cursor rule or skill directory.
4. Future agent-specific directories.

The implementation should treat symlinks as generated artifacts. `doctor` should be able to detect broken or incorrect symlinks, and `sync` should be able to restore them.

## Preset Skill Model

Codeseed stores built-in preset skills in `presets/skills/`.

Preset skills are part of the normal skill source model:

1. `codeseed init` installs the default preset set unless `--no-presets` is used.
2. `codeseed add preset:<skill-id>` installs a preset skill explicitly.
3. `codeseed rm <skill>` removes an installed preset skill the same way it removes any other managed skill.
4. `codeseed doctor` validates installed preset skills with the same manifest rules as other skills.

Each preset skill should include `skill.toml` and a single `SKILL.md`. Current preset skills use Chinese content in `SKILL.md` instead of parallel localized skill documents.

## Skill Source Model

Every skill source should be normalized into an internal resolved skill package before installation.

Suggested source flow:

```text
input source
  -> source resolver
  -> fetched or local package
  -> manifest parser
  -> validator
  -> placement planner
  -> installer
  -> state recorder
```

This keeps `add` behavior consistent whether the skill comes from Codeseed presets, SkillHub, GitHub, a URL, or a local path.

## State And Git Model

Codeseed should make local state explicit enough that Git can help synchronize it.

`.codeseed/` should likely record:

1. Installed skill id, version, and source.
2. Content hash or revision.
3. Target placement.
4. Generated symlink plan.
5. Operation history or install lock data.

Open question: decide which `.codeseed/` files should be committed. A likely starting point is to commit deterministic desired state, such as `state.json` or `lock.json`, while ignoring transient cache files.

## First Implementation Milestone

The first useful milestone can be intentionally small:

1. Provide `codeseed init`.
2. Create `.agent/` and `.codeseed/`.
3. Install one or two bundled preset skills.
4. Create compatibility symlinks.
5. Provide `codeseed doctor` for the initialized structure.
6. Add tests around idempotent init and doctor diagnostics.

After that, `add`, `rm`, source resolvers, and `sync` can be added incrementally.

## Open Design Questions

1. Should the canonical agent directory be `.agent` or `.agents`?
2. Should agent compatibility directories live inside `.agent/`, or should Codeseed create symlinks into tool-native paths at the project root?
3. What is the first supported skill manifest format?
4. Should Codeseed define its own manifest, adapt existing agent skill manifests, or support both?
5. Which files in `.codeseed/` should be committed by default?
6. Should `sync` be purely local and Git-backed at first, or should it later connect to SkillHub state?
7. How strict should `doctor` be when it finds user-created files inside managed directories?
