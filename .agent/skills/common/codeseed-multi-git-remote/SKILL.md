---
name: codeseed-multi-git-remote
description: Manage multiple Git remotes for one repository, including adding, removing, fetching, pulling, and pushing across remotes such as GitHub and Gitee.
license: MIT
compatibility: Designed for Codeseed-managed project skills and Agent Skills compatible clients.
metadata:
  codeseed.version: "0.1.0"
---

# Codeseed Multi Git Remote

Use this skill when a repository is mirrored across multiple Git remotes and the user wants help keeping those remotes configured and synchronized.

Core default: in a mirrored repository, an unqualified Git push request means push the current branch to every configured push remote. This rule should be discoverable from repository instructions and should not depend on an agent remembering prior conversation.

## Scope

This skill covers:

1. Inspecting configured Git remotes.
2. Adding a remote, for example `github`, `gitee`, `origin`, or another mirror.
3. Removing or renaming a remote.
4. Fetching from multiple remotes.
5. Pulling from a selected remote and branch.
6. Pushing the current branch or a named branch to multiple remotes.
7. Verifying that remote URLs, upstream tracking, and branch state are consistent.

## Safety Rules

1. Always inspect `git remote -v`, `git branch -vv`, and `git status --short` before changing remote configuration.
2. Do not remove or rename a remote unless the user explicitly asks for that remote by name.
3. Do not rewrite history, force-push, reset, or delete remote branches unless the user explicitly requests that exact destructive action.
4. Prefer non-interactive Git commands.
5. When pushing to multiple remotes, push one remote at a time and report which remote failed if any command fails.
6. If the working tree is dirty, warn before pulling or rebasing.
7. In a mirrored repository, when the user asks to `push`, "save and push", or otherwise publish work without naming a single remote, treat all configured push-capable remotes as the intended targets.
8. If a remote rejects a push because it is not a fast-forward, fetch and inspect that remote before integrating; never force-push unless explicitly requested.

## Common Commands

Inspect remotes:

```bash
git remote -v
git branch -vv
git status --short
```

Add remotes:

```bash
git remote add github git@github.com:OWNER/REPO.git
git remote add gitee git@gitee.com:OWNER/REPO.git
```

Remove a remote:

```bash
git remote remove REMOTE
```

Fetch multiple remotes:

```bash
git fetch github
git fetch gitee
```

Push the current branch to multiple remotes:

```bash
git push origin HEAD
git push gitee HEAD
```

Push a named branch to multiple remotes and set upstream when appropriate:

```bash
git push -u origin BRANCH
git push gitee BRANCH
```

## Recommended Workflow

1. Confirm the current branch and working tree state.
2. Confirm the intended remote names and URLs.
3. Add, remove, or update remotes as requested.
4. Fetch from all relevant remotes.
5. Compare branch tracking state.
6. If the user named one remote, push or pull only that remote. If they did not name one remote and the repository has multiple configured push remotes, push each relevant remote one at a time.
7. Summarize each remote action and its result.
