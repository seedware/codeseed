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

## Skill Routing

Git workflow behavior is declared by installed skills under `.agent/skills/`. Follow the generic skill activation rules in [AGENTS.md](../../AGENTS.md) instead of hard-coding individual skill names in this context file.
