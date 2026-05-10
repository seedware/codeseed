# Codeseed CLI

英文版本：[cli.md](cli.md)。

本文档描述 Codeseed 初始命令行界面。当前实现采用 Rust CLI，并使用 `clap` 定义命令。

## 全局选项

```text
codeseed --project <DIR> <COMMAND>
```

选项：

1. `--project <DIR>`：要操作的项目根目录。默认是当前目录。

## 命令

### `codeseed init`

在项目中初始化 Codeseed。

```text
codeseed init [--agent-dir <DIR>] [--codeseed-dir <DIR>] [--no-presets] [--no-links] [--force]
```

选项：

1. `--agent-dir <DIR>`：创建并管理的 agent-facing 目录。默认是 `.agent`。
2. `--codeseed-dir <DIR>`：Codeseed 元数据目录。默认是 `.codeseed`。
3. `--no-presets`：跳过内置预置 skills。
4. `--no-links`：跳过已知 agent 的兼容软链接。
5. `--force`、`-f`：在可行时覆盖不兼容的 Codeseed 生成状态。

### `codeseed add <SOURCE>`

从 Codeseed 预置 skills、SkillHub、GitHub、URL、文件或目录添加 skill。

```text
codeseed add <SOURCE> [--hub <URL>] [--name <NAME>] [--target <TARGET>] [--target-dir <DIR>] [--force]
```

来源可以是 `preset:<skill-id>`、SkillHub id、GitHub reference、URL、本地文件或本地目录。

选项：

1. `--hub <URL>`：从指定 SkillHub endpoint 解析来源。
2. `--name <NAME>`：使用指定本地名称安装 skill。
3. `--target <TARGET>`：覆盖从 skill manifest 推断出的放置目标。
4. `--target-dir <DIR>`：覆盖目标目录。
5. `--force`、`-f`：替换已有同 id 或同名的受管理 skill。

目标：

1. `common`
2. `codex`
3. `claude`
4. `cursor`

### `codeseed list`

列出可用的内置 skills，或列出当前项目已安装的 skills。

```text
codeseed list [--installed] [--format <FORMAT>]
```

选项：

1. `--installed`：列出当前项目已安装的 skills，而不是内置可用 skills。
2. `--format <FORMAT>`：输出格式。默认是 `text`。

格式：

1. `text`
2. `json`

### `codeseed update`

更新 `codeseed` 可执行文件本身。

```text
codeseed update [--version <VERSION>] [--home <DIR>] [--bin-dir <DIR>] [--mode <MODE>] [--script-url <URL>] [--dry-run]
```

行为：

1. 如果当前项目中存在本地 `scripts/install.sh`，Codeseed 会使用它。
2. 否则 Codeseed 会下载安装脚本，然后使用 `sh` 执行。
3. 安装器默认安装到 `~/.codeseed/bin`。

选项：

1. `--version <VERSION>`：要安装的 Codeseed 版本。默认是 `latest`。
2. `--home <DIR>`：Codeseed 主目录。安装器中默认是 `~/.codeseed`。
3. `--bin-dir <DIR>`：可执行文件目录。安装器中默认是 `~/.codeseed/bin`。
4. `--mode <MODE>`：安装策略。默认是 `auto`。
5. `--script-url <URL>`：当没有本地安装器时使用的安装脚本 URL。
6. `--dry-run`：只展示更新计划，不执行更新。

模式：

1. `auto`
2. `local`
3. `prebuilt`

### `codeseed rm <SKILL>`

移除已安装的 Codeseed-managed skill。

```text
codeseed rm <SKILL> [--force] [--prune]
```

别名：

1. `codeseed remove <SKILL>`

选项：

1. `--force`、`-f`：即使本地状态部分不一致，也移除生成文件。
2. `--prune`：移除 skill 后，同时移除空的生成父目录。

### `codeseed doctor`

诊断 Codeseed 状态、skill manifest 和兼容软链接。

```text
codeseed doctor [--strict] [--fix] [--format <FORMAT>]
```

选项：

1. `--strict`：把 warning 当作 failure。
2. `--fix`：尝试安全修复生成目录和兼容软链接。
3. `--format <FORMAT>`：输出格式。默认是 `text`。

格式：

1. `text`
2. `json`

### `codeseed sync`

根据已记录的 Codeseed 状态协调生成文件。

```text
codeseed sync [--dry-run] [--check] [--prune]
```

选项：

1. `--dry-run`：只展示计划变更，不修改文件。
2. `--check`：只检查生成文件是否匹配记录状态，不修复。
3. `--prune`：当过期生成文件仍归 Codeseed 所有时，移除它们。

`--dry-run` 和 `--check` 互斥。

### `codeseed clear`

从项目中清除 Codeseed 管理的状态和生成的 agent 内容。

```text
codeseed clear [--agent-dir <DIR>] [--codeseed-dir <DIR>] [--dry-run] [--yes --confirm clear-codeseed-state]
```

这是一个破坏性命令。默认情况下，Codeseed 必须要求用户进行交互式二次确认，然后才会移除任何内容。

选项：

1. `--agent-dir <DIR>`：要移除的 agent-facing 目录。默认是 `.agent`。
2. `--codeseed-dir <DIR>`：要移除的 Codeseed 元数据目录。默认是 `.codeseed`。
3. `--dry-run`：只展示会移除什么，不修改文件。
4. `--yes`：跳过交互式确认提示。
5. `--confirm clear-codeseed-state`：使用 `--yes` 时必须提供的确认短语。

`--yes` 和 `--confirm clear-codeseed-state` 必须一起使用，并且不能和 `--dry-run` 同时使用。
