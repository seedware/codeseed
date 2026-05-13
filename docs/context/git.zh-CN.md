# Git 上下文

这个仓库同时镜像到 GitHub 和 Gitee。

## Remotes

当前预期 remotes：

```text
origin  git@github.com:seedware/codeseed.git
gitee   git@gitee.com:seedware/codeseed.git
```

本地 `main` 分支当前跟踪 `gitee/main`。

## 提交实践

1. 保持 commit 聚焦且描述清楚。
2. 使用历史中已有的 conventional-style message，例如 `feat: ...` 或 `docs: ...`。
3. 不提交无关 dirty work。
4. 修改 Rust 行为的提交前运行测试。

## Skill 路由

Git workflow 行为由 `.agent/skills/` 下已安装的 skills 声明。遵循 [AGENTS.md](../../AGENTS.md) 中的通用 skill 激活规则，不要在这个 context 文件中硬编码单个 skill 名称。
