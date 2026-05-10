# Codeseed Skill Author

当需要为 Codeseed 管理的项目创建、审查或改进 agent skills 时，使用这个 skill。

## 工作流程

1. 识别 skill 面向的目标 agent。
2. 让 skill 聚焦于一个可重复使用的能力。
3. 定义预期文件、入口文档和放置目标。
4. 为每个面向用户的 Markdown 文档提供中文版本。
5. 优先提供可以被 `codeseed doctor` 验证的小例子。
6. 除非 skill 明确记录远程依赖，否则避免隐藏依赖外部 SkillHub。

## 输出预期

产出 skill 时，应包含：

1. `skill.toml`
2. `SKILL.md`
3. `SKILL.zh-CN.md`
4. 所有被引用的 assets 或 scripts

这个 skill 应该可以先从本地目录安装，然后再考虑发布到其它地方。

