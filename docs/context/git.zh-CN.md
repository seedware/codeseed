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

执行 push、pull、fetch、add remote 或 remove remote 前：

```bash
git status --short
git remote -v
git branch -vv
```

向多个 remotes push 时逐个执行；如果失败，说明失败的是哪个 remote。

