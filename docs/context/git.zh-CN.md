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

## 多 Remote 工作

remote 操作使用已安装的 `codeseed-multi-git-remote` skill。

如果用户要求 push 或保存并推送，但没有指定单个 remote，应将当前分支推送到这个镜像仓库的所有已配置 push remotes。逐个 remote 执行 push。

执行 push、pull、fetch、add remote 或 remove remote 前：

```bash
git status --short
git remote -v
git branch -vv
```

如果失败，说明失败的是哪个 remote。如果某个 remote 拒绝非 fast-forward push，先 fetch 并检查该 remote，再做集成；除非用户明确要求，否则不要 force-push。
