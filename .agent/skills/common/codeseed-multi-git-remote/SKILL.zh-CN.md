---
name: codeseed-multi-git-remote
description: 管理同一个仓库的多个 Git 远程仓库，包括添加、删除、fetch、pull，以及向 GitHub、Gitee 等多个 remote push。
license: MIT
compatibility: 适用于 Codeseed 管理的项目 skills 以及兼容 Agent Skills 的客户端。
metadata:
  codeseed.version: "0.1.0"
---

# Codeseed Multi Git Remote

当一个仓库同时镜像到多个 Git remote，并且用户希望配置、检查或同步这些 remote 时，使用这个 skill。

## 范围

这个 skill 覆盖：

1. 检查已配置的 Git remotes。
2. 添加 remote，例如 `github`、`gitee`、`origin` 或其它镜像。
3. 删除或重命名 remote。
4. 从多个 remotes 执行 fetch。
5. 从指定 remote 和 branch 执行 pull。
6. 将当前分支或指定分支 push 到多个 remotes。
7. 验证 remote URL、upstream tracking 和 branch 状态是否一致。

## 安全规则

1. 修改 remote 配置前，必须先检查 `git remote -v`、`git branch -vv` 和 `git status --short`。
2. 除非用户明确指定 remote 名称，否则不要删除或重命名 remote。
3. 除非用户明确要求具体的破坏性操作，否则不要 rewrite history、force-push、reset 或删除远程分支。
4. 优先使用非交互式 Git 命令。
5. 向多个 remotes push 时，应逐个 remote 执行；如果某个命令失败，需要说明失败的是哪个 remote。
6. 如果工作区有未提交变更，在 pull 或 rebase 前必须提醒用户。

## 常用命令

检查 remotes：

```bash
git remote -v
git branch -vv
git status --short
```

添加 remotes：

```bash
git remote add github git@github.com:OWNER/REPO.git
git remote add gitee git@gitee.com:OWNER/REPO.git
```

删除 remote：

```bash
git remote remove REMOTE
```

从多个 remotes fetch：

```bash
git fetch github
git fetch gitee
```

将当前分支 push 到多个 remotes：

```bash
git push github HEAD
git push gitee HEAD
```

将指定分支 push 到多个 remotes，并在合适时设置 upstream：

```bash
git push -u github BRANCH
git push gitee BRANCH
```

## 推荐流程

1. 确认当前分支和工作区状态。
2. 确认目标 remote 名称和 URL。
3. 按用户要求添加、删除或更新 remotes。
4. 从所有相关 remotes fetch。
5. 对比 branch tracking 状态。
6. 只有在目标 branch 和 remote 明确后，才执行 push 或 pull。
7. 总结每个 remote 操作和结果。

