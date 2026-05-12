# Skill 目录结构

英文版本：[skill-layout.md](skill-layout.md)。

Codeseed 使用一个项目内的规范 skill 布局，然后为特定 agent 工具生成兼容入口。

## 现有约定

当前生态还没有一个所有 agent 都统一使用的项目目录，但已经有比较明确的共同形态：

1. 兼容 Agent Skills 的工具通常把一个 skill 视为一个目录，其中必须包含 `SKILL.md`，并可选包含 `scripts/`、`references/` 和 `assets/`。
2. Claude Code 的项目级 skills 位于 `.claude/skills/<skill-name>/`。
3. Cursor 的项目规则位于 `.cursor/rules/*.mdc`；Cursor 也支持根目录 `AGENTS.md` 作为简单项目指令。
4. Codex 和许多 coding agents 会把 `AGENTS.md` 作为项目指令文件。

## Codeseed 规范布局

```text
.
├── .agent/
│   ├── skills/
│   │   ├── common/
│   │   │   └── <skill-id>/
│   │   │       ├── skill.toml
│   │   │       ├── SKILL.md
│   │   │       └── SKILL.zh-CN.md
│   │   ├── codex/
│   │   ├── claude/
│   │   └── cursor/
│   └── generated/
│       └── cursor-rules/
├── .codeseed/
│   └── state.json
├── .claude/
│   └── skills/
├── .cursor/
│   └── rules/
└── AGENTS.md
```

规则：

1. `.agent/skills/` 是 Codeseed 拥有的规范 skill 树。
2. `.agent/skills/common/` 存放可以暴露给多个 agent 的通用 skills。
3. agent-specific 目录预留给只适用于单个 agent 的 skills 或 adapters。
4. `.agent/generated/` 存放生成的兼容产物。
5. `.codeseed/` 记录期望状态，供未来 `sync` 使用。
6. `.claude/skills/` 和 `.cursor/rules/` 这类工具原生目录应从 Codeseed 状态生成。

## 兼容策略

Claude Code：

```text
.claude/skills/<skill-id> -> .agent/skills/common/<skill-id>
```

Cursor：

```text
.cursor/rules/<skill-id>.mdc -> .agent/generated/cursor-rules/<skill-id>.mdc
```

Codex 和通用 coding agents：

```text
AGENTS.md
```

`AGENTS.md` 会指向 Codeseed 管理的 skill 树，并记录仓库级操作指引。

## Skill 包形态

每个 skill 目录应兼容 Agent Skills 形态：

```text
<skill-id>/
├── SKILL.md
├── skill.toml
├── SKILL.zh-CN.md
├── scripts/
├── references/
└── assets/
```

广义 Agent Skills 约定只要求 `SKILL.md`。Codeseed 额外使用 `skill.toml` 记录本地管理元数据；当 skill 有面向用户的文档时，也保留 `SKILL.zh-CN.md`。

## 刷新项目 Skills

项目 skills 存放在项目目录内，所以更新全局的 `codeseed` 可执行文件不会自动改变已经初始化过的项目。请在项目根目录刷新项目内容。

对于当前已实现的 preset 来源，使用 `--force` 重新安装该 preset：

```bash
codeseed add preset:<skill-id> --force
```

例如：

```bash
codeseed add preset:codeseed-context-index --force
```

刷新后，检查已安装 skill 列表和生成的兼容入口：

```bash
codeseed list --installed
ls -l .claude/skills .cursor/rules
```

对于 Cursor rules，每个 `.cursor/rules/<skill-id>.mdc` 都应该是指向 `.agent/generated/cursor-rules/<skill-id>.mdc` 的软链接。对于 Claude Code，每个 `.claude/skills/<skill-id>` 都应该指向 `.agent/skills/common/<skill-id>`。

如果当前安装版本已经支持这些命令，`codeseed doctor --fix` 可以安全修复生成目录或兼容软链接问题，`codeseed sync --check` 可以在不改写文件的情况下检查记录的生成状态。
