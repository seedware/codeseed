---
name: codeseed-context-index
description: Maintain docs/context as a compact project context index so new model threads can quickly discover architecture, constraints, module design, Git rules, reusable components, and other working context.
license: MIT
compatibility: Designed for Codeseed-managed project skills and Agent Skills compatible clients.
metadata:
  codeseed.version: "0.1.0"
---

# Codeseed Context Index

Use this skill when creating or maintaining project context for AI-assisted development.

The goal is to make new threads productive without requiring the user to repeat background information. The `docs/context/` directory should act as a compact index that points to the right detailed documents.

## Responsibilities

1. Create `docs/context/` during project initialization.
2. Keep `docs/context/README.md` as the primary context index.
3. Keep the index concise: it should tell the model what to read, not duplicate every detail.
4. Link to detailed documents for architecture, module design, code constraints, Git requirements, reusable components, and operational notes.
5. Update the context index when project structure, conventions, or important decisions change.
6. Prefer editing existing docs when they are the canonical source, then update the context index to point at them.

## Global Rule

Any project that installs this skill adopts this convention:

1. At the start of a new model thread, read `docs/context/README.md` before making assumptions about the project.
2. If `docs/context/README.md` links to task-relevant context, read only those linked files needed for the task.
3. If `docs/context/` is missing or stale, create or repair it before relying on project memory.
4. Treat this rule as project-wide guidance, not a Codeseed-repository-only convention.

## Suggested Files

Use only the files that are useful for the project:

1. `docs/context/README.md`: compact index and reading order.
2. `docs/context/architecture.md`: system and framework design entry points.
3. `docs/context/modules.md`: module boundaries and responsibilities.
4. `docs/context/constraints.md`: coding rules, design constraints, and testing expectations.
5. `docs/context/git.md`: branch, remote, commit, and release expectations.
6. `docs/context/components.md`: reusable components, helpers, or internal APIs.

## Maintenance Rules

1. Keep `docs/context/README.md` short enough to read at the start of every thread.
2. Put stable details in dedicated files and link to them from the index.
3. Avoid copying large sections of source code into context docs.
4. Prefer clear relative paths when pointing to important project files.
5. When a document becomes stale, update or remove the index entry.
6. Every user-facing Markdown context document should have a Chinese version when practical.

## New Thread Protocol

When starting work in a project that uses this skill:

1. Read `docs/context/README.md` first.
2. Follow its links only for the parts relevant to the current task.
3. Check `AGENTS.md` when present.
4. Update context docs when the task changes durable project knowledge.
