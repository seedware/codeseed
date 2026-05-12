# Git Context

This repository is mirrored to GitHub and Gitee.

## Remotes

Current expected remotes:

```text
origin  git@github.com:seedware/codeseed.git
gitee   git@gitee.com:seedware/codeseed.git
```

The local `main` branch currently tracks `gitee/main`.

## Commit Practice

1. Keep commits focused and descriptive.
2. Use conventional-style messages already present in history, for example `feat: ...` or `docs: ...`.
3. Do not commit unrelated dirty work.
4. Run tests before commits that change Rust behavior.

## Multi-Remote Work

Use the installed `codeseed-multi-git-remote` skill for remote operations.

Before push, pull, fetch, add, or remove remote work:

```bash
git status --short
git remote -v
git branch -vv
```

Push remotes one at a time and report which remote fails if any command fails.

