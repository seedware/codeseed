# 组件上下文

这个文件列出可复用的项目部件，以及新增内容前应该先看的位置。

## CLI 类型

所有命令参数类型都放在 `src/cli.rs`：

1. 命令枚举：`Command`；
2. 命令结构体：`InitCommand`、`AddCommand`、`ListCommand`、`UpdateCommand` 等；
3. 共享枚举：`OutputFormat`、`SkillTarget`、`UpdateMode`。

## 错误处理

使用 `src/error.rs` 中的 `CodeseedError`。

当前 variants：

1. `Io`：带路径或上下文的文件系统/进程 IO 失败；
2. `Conflict`：面向用户的冲突或尚不支持的状态。

## Presets

Preset 元数据位于 `src/presets.rs`。

Preset 内容位于 `presets/skills/<skill-id>/`。

当前 preset skills：

1. `codeseed-skill-author`
2. `codeseed-context-index`
3. `codeseed-chinese-code-comments`
4. `codeseed-multi-git-remote`
5. `codeseed-prebuilt-release`

## 生成的兼容入口

兼容入口生成逻辑当前位于 `src/init.rs`：

1. Claude links：`.claude/skills/<skill-id>`；
2. Cursor rules：`.cursor/rules/<skill-id>.mdc`；
3. 通用 agent 指令：`AGENTS.md`。

新增生成逻辑前，优先复用这些 helper。
