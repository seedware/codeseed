# Codeseed

Codeseed 是一个用于在项目中管理 agent skills 的命令行工具。

当前项目阶段是产品和架构探索。初始项目说明见 [docs/project-brief.zh-CN.md](docs/project-brief.zh-CN.md)。
初始命令行界面见 [docs/cli.zh-CN.md](docs/cli.zh-CN.md)。
项目内 skill 目录结构见 [docs/skill-layout.zh-CN.md](docs/skill-layout.zh-CN.md)。
内置预置 skills 存放在 [presets/](presets/)。

## 规划方向

Codeseed 会先从一个本地 CLI 开始：它可以初始化 agent skill 目录，安装预置 skill，为常见 agent 创建兼容软链接，诊断已安装的 skill，并记录足够确定性的状态，以便通过 Git 在多个本地检出目录之间同步 skill 配置。

未来的 SkillHub 服务不属于第一个里程碑范围，但 CLI 的来源模型需要提前设计好，让后续接入 SkillHub 时不需要改动核心安装流程。
