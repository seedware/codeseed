# Codeseed

Codeseed 是一个用于在项目中管理 agent skills 的命令行工具。

当前项目阶段是产品和架构探索。初始项目说明见 [docs/project-brief.zh-CN.md](docs/project-brief.zh-CN.md)。
初始命令行界面见 [docs/cli.zh-CN.md](docs/cli.zh-CN.md)。
安装方式见 [docs/install.zh-CN.md](docs/install.zh-CN.md)。
项目内 skill 目录结构见 [docs/skill-layout.zh-CN.md](docs/skill-layout.zh-CN.md)。
内置预置 skills 存放在 [presets/](presets/)。

## 安装

```bash
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh | sh
```

## 更新

更新 `codeseed` 可执行文件本身：

```bash
codeseed update --dry-run
codeseed update
```

刷新项目中已安装的预置 skill：

```bash
codeseed add preset:<skill-id> --force
```

然后检查项目内生成入口：

```bash
codeseed list --installed
ls -l .claude/skills .cursor/rules
```

更多细节见 [docs/install.zh-CN.md](docs/install.zh-CN.md) 中的 Codeseed 本体更新说明，以及 [docs/skill-layout.zh-CN.md](docs/skill-layout.zh-CN.md) 中的项目 skill 和兼容软链接刷新说明。

## 规划方向

Codeseed 会先从一个本地 CLI 开始：它可以初始化 agent skill 目录，安装预置 skill，为常见 agent 创建兼容软链接，诊断已安装的 skill，并记录足够确定性的状态，以便通过 Git 在多个本地检出目录之间同步 skill 配置。

未来的 SkillHub 服务不属于第一个里程碑范围，但 CLI 的来源模型需要提前设计好，让后续接入 SkillHub 时不需要改动核心安装流程。
