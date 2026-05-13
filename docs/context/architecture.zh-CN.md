# 架构上下文

Codeseed 是一个 Rust CLI，用于管理项目内的 agent skills。

## 产品形态

Codeseed 当前聚焦本地项目操作：

1. 初始化项目内 skill 布局。
2. 安装内置预置 skills。
3. 为常见 agent 生成兼容入口。
4. 在 `.codeseed/state.json` 中记录期望的 skill 状态。
5. 提供 list、add、update 等 CLI 命令，后续继续补齐 rm、doctor、sync、clear 行为。

未来 SkillHub 不属于当前仓库范围，但命令和 source resolution 设计应为它保留空间。

## 规范项目布局

规范 skill 树是 `.agent/skills/`。

当前生成的项目布局：

```text
.agent/
  skills/common/<skill-id>/
  generated/cursor-rules/<skill-id>.mdc
.codeseed/state.json
.claude/skills/<skill-id> -> ../../.agent/skills/common/<skill-id>
.cursor/rules/<skill-id>.mdc -> ../../.agent/generated/cursor-rules/<skill-id>.mdc
AGENTS.md
docs/context/
```

工具原生目录是生成的兼容入口。`.agent/skills/common/` 是已安装 skill 内容的规范来源。

## 状态模型

`.codeseed/state.json` 当前记录：

1. schema version；
2. agent 目录；
3. Codeseed 元数据目录；
4. 已安装 skill 的 id、source 和 target。

保持该文件确定、可审查。它预期被提交到 Git。

## 安装与更新

安装使用 [scripts/install.sh](../../scripts/install.sh)。默认安装到 `~/.codeseed/bin`，并创建 `~/.codeseed/config` 和 `~/.codeseed/cache`。

`codeseed update` 复用安装脚本：

1. 本地存在 `scripts/install.sh` 时使用本地脚本；
2. 否则下载已发布脚本；
3. 支持 `--dry-run`，让用户先检查执行计划。
