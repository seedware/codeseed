# Codeseed 项目说明

## 项目定位

Codeseed 是一个命令行工具，用于管理目标项目中的 agent skills。

当前阶段，Codeseed 应优先聚焦于本地项目初始化和 skill 生命周期管理。长期方向是成为未来 SkillHub 生态的本地配套工具，同时即使没有任何远程服务也能保持可用。

## 目标

1. `codeseed init` 用于在当前目录初始化 agent skill 管理能力。
2. 初始化时创建 `.agent` 目录，以及必要的 skill 目录结构。
3. 初始化时从仓库内的 `presets/skills/` 目录安装少量内置预置 skill。
4. 初始化时为常见 agent 创建兼容软链接，包括 Claude Code、Codex、Cursor，以及类似工具。
5. 后续通过 `codeseed add` 和 `codeseed rm` 管理 skill。
6. skill 的放置位置由 skill 配置驱动，因此 Codeseed 可以把每个 skill 放到正确的目标目录。
7. skill 来源需要支持：
   - 默认 SkillHub；
   - 用户指定的 SkillHub；
   - 来自 URL 的单个 skill 文件；
   - 本地单个 skill 文件；
   - 本地 skill 目录；
   - GitHub 仓库或仓库内路径。
8. `codeseed doctor` 用于验证所有已安装 skill，并报告无效或不兼容的 skill。
9. Codeseed 需要和 Git 良好配合。
10. Codeseed 需要记录足够的本地操作状态，以支持 `codeseed sync` 在同一项目的不同本地目录中同步 Codeseed 状态。

## 第一个里程碑的非目标

1. 构建 SkillHub 本身。
2. 实现远程发布、评分、搜索或账号功能。
3. 第一天就支持所有可能的 agent-specific skill 格式。
4. 对用户隐藏生成文件。生成的 `.agent` 状态应该是可理解、可审查的。

## 初始命令面

### `codeseed init`

创建本地 Codeseed 管理的 agent skill 结构。

预期行为：

1. 创建 `.agent/`。
2. 创建必要的 skill 目录。
3. 安装内置预置 skill。
4. 为 agent 工具创建兼容软链接。
5. 创建 `.codeseed/` 元数据，用于本地状态记录和未来同步。
6. 遇到不兼容的已有状态时拒绝覆盖，除非用户显式传入 force 选项。

### `codeseed add <source>`

从受支持来源添加一个 skill。

预期来源类型：

1. 默认 SkillHub 标识，例如 `codeseed add test-writer`。
2. 显式 SkillHub 标识，例如 `codeseed add --hub <url> test-writer`。
3. 远程 skill 文件 URL。
4. 本地 skill 文件路径。
5. 本地 skill 目录路径。
6. GitHub 仓库或仓库内路径。

预期行为：

1. 解析来源。
2. 读取 skill 配置。
3. 验证 skill。
4. 将文件放置到正确的受管理目录。
5. 在 `.codeseed/` 中记录操作。

### `codeseed list`

列出 Codeseed 已知的 skills。

预期行为：

1. 默认列出内置预置 skills。
2. 支持列出当前项目已安装的 skills。
3. 支持便于脚本使用的机器可读输出。
4. 未来可以扩展到 SkillHub-backed 来源，而不改变命令形态。

### `codeseed update`

更新 Codeseed 可执行文件本身。

预期行为：

1. 当命令在 Codeseed 源码仓库中运行时，使用本地安装脚本。
2. 否则下载并运行已发布的安装脚本。
3. 支持选择版本。
4. 支持 dry-run 输出，让用户先检查更新计划。
5. 复用安装时的 `~/.codeseed` 主目录模型。

### `codeseed rm <skill>`

移除一个受管理的 skill。

预期行为：

1. 通过名称或 id 查找已安装的 skill。
2. 移除受管理文件。
3. 保留用户拥有的文件，除非这些文件被 Codeseed 明确管理。
4. 在 `.codeseed/` 中记录操作。

### `codeseed doctor`

验证当前 Codeseed 和 skill 状态。

预期检查：

1. `.agent/` 存在，并匹配预期结构。
2. 兼容软链接存在，并指向预期位置。
3. 已安装 skill 的 manifest 有效。
4. manifest 引用的 skill 文件存在。
5. agent-specific 输出目录保持一致。
6. `.codeseed/` 状态可读取，并且内部一致。

### `codeseed sync`

在同一 Git 项目的不同本地目录中同步 Codeseed 管理的本地状态。

预期行为：

1. 读取 `.codeseed/` 状态。
2. 对比期望的已安装 skill 状态和当前 `.agent/` 内容。
3. 重新创建缺失的生成文件和软链接。
4. 在安全时移除过期的生成文件。
5. 保持行为确定性，让 Git 可以跟踪有意义的变化。

### `codeseed clear`

从目标项目中清除 Codeseed 管理的状态和生成的 agent 内容。

预期行为：

1. 把这个命令视为破坏性命令。
2. 支持 `--dry-run`，让用户先检查移除计划。
3. 默认删除文件前必须要求用户进行交互式二次确认。
4. 非交互式执行时必须同时提供 `--yes` 和 `--confirm clear-codeseed-state`。
5. 只移除 Codeseed 拥有的元数据和生成的 agent-facing 内容。
6. 避免删除未记录为 Codeseed-managed 的用户文件。

## 建议目录模型

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

具体目录名仍可讨论。核心区分是：

1. `.agent/` 包含面向 agent 的 skill 文件和兼容布局。
2. `.codeseed/` 包含 Codeseed 拥有的元数据，用来说明安装了什么、来源是什么，以及如何复现。

## 兼容软链接意图

Codeseed 应该把同一份底层受管理 skill 内容暴露给多个 agent 工具，避免用户手动重复复制文件。

可能的兼容目标：

1. Claude Code skill 目录。
2. Codex skill 目录。
3. Cursor rule 或 skill 目录。
4. 未来 agent-specific 目录。

实现时应把软链接视为生成产物。`doctor` 应能检测损坏或错误的软链接，`sync` 应能恢复它们。

## 预置 Skill 模型

Codeseed 把内置预置 skills 存放在 `presets/skills/`。

预置 skills 是普通 skill 来源模型的一部分：

1. `codeseed init` 默认安装预置集合，除非用户使用 `--no-presets`。
2. `codeseed add preset:<skill-id>` 可以显式安装某个预置 skill。
3. `codeseed rm <skill>` 可以像移除其它受管理 skill 一样移除已安装的预置 skill。
4. `codeseed doctor` 使用与其它 skill 相同的 manifest 规则验证已安装的预置 skill。

每个预置 skill 都应该包含 `skill.toml` 和唯一的 `SKILL.md`。当前预置 skills 在 `SKILL.md` 中使用中文内容，而不是维护并行的多语言 skill 文档。

## Skill 来源模型

每一种 skill 来源都应该先被标准化为内部 resolved skill package，然后再安装。

建议来源流程：

```text
输入来源
  -> 来源解析器
  -> 已拉取或本地 package
  -> manifest 解析器
  -> 验证器
  -> 放置规划器
  -> 安装器
  -> 状态记录器
```

这样无论 skill 来自 Codeseed 预置集合、SkillHub、GitHub、URL 还是本地路径，`add` 行为都能保持一致。

## 状态与 Git 模型

Codeseed 应该让本地状态足够显式，使 Git 可以帮助同步它。

`.codeseed/` 可能需要记录：

1. 已安装 skill 的 id、版本和来源。
2. 内容 hash 或 revision。
3. 目标放置位置。
4. 生成软链接计划。
5. 操作历史或安装 lock 数据。

待讨论问题：决定 `.codeseed/` 中哪些文件应该提交到 Git。一个可能的起点是提交确定性的期望状态，例如 `state.json` 或 `lock.json`，同时忽略临时 cache 文件。

## 第一个实现里程碑

第一个可用里程碑可以刻意保持很小：

1. 提供 `codeseed init`。
2. 创建 `.agent/` 和 `.codeseed/`。
3. 安装一到两个内置预置 skill。
4. 创建兼容软链接。
5. 提供针对初始化结构的 `codeseed doctor`。
6. 为 init 的幂等性和 doctor 诊断添加测试。

之后再逐步加入 `add`、`rm`、来源解析器和 `sync`。

## 开放设计问题

1. 规范 agent 目录应该叫 `.agent` 还是 `.agents`？
2. agent 兼容目录应该放在 `.agent/` 内，还是 Codeseed 应该在项目根目录创建指向工具原生路径的软链接？
3. 第一个受支持的 skill manifest 格式是什么？
4. Codeseed 应该定义自己的 manifest、适配现有 agent skill manifest，还是两者都支持？
5. `.codeseed/` 中哪些文件默认应该被提交？
6. `sync` 一开始应该是纯本地、基于 Git 的，还是未来也要连接 SkillHub 状态？
7. 当 `doctor` 在受管理目录中发现用户创建的文件时，应该有多严格？
