# Codeseed 预置 Skills

英文版本：[README.md](README.md)。

这个目录用于存放随 Codeseed 一起发布的内置 skills。

预置 skills 有两个用途：

1. `codeseed init` 可以把它们自动安装到目标项目中。
2. Codeseed 开发阶段可以在不依赖外部 SkillHub 的情况下验证 `add`、`rm`、`doctor` 和 `sync` 行为。

## 目录结构

```text
presets/
└── skills/
    └── <skill-id>/
        ├── skill.toml
        └── SKILL.md
```

## 来源语法

预置 skills 应该可以通过普通 `add` 流程使用显式来源前缀引用：

```text
codeseed add preset:<skill-id>
```

例如：

```text
codeseed add preset:codeseed-skill-author
codeseed add preset:codeseed-context-index
codeseed add preset:codeseed-chinese-code-comments
codeseed add preset:codeseed-multi-git-remote
codeseed add preset:codeseed-prebuilt-release
```

## Manifest

每个预置 skill 都必须包含 `skill.toml`。第一个 manifest 版本刻意保持很小：

```toml
schema = 1
id = "codeseed-skill-author"
name = "Codeseed Skill Author"
version = "0.1.0"
target = "common"
entry = "SKILL.md"
```

`target` 使用与 CLI 相同的目标名称：`common`、`codex`、`claude` 或 `cursor`。
