---
name: codeseed-prebuilt-release
description: 发布安装脚本使用的 Codeseed 预编译 release 包。用于构建、打包、上传或验证 codeseed-<target>.tar.gz release asset，初始阶段只覆盖当前宿主平台。
license: MIT
compatibility: 适用于使用 scripts/install.sh 下载预编译包的 Codeseed 发布流程。
metadata:
  codeseed.version: "0.1.0"
---

# Codeseed 预编译发布

当需要发布 `scripts/install.sh` 使用的 Codeseed 预编译压缩包时，使用这个 skill。

这个流程刻意先只覆盖当前宿主平台。在跨平台打包补齐前，不要暗示其他平台已经构建完成。

## 压缩包约定

`scripts/install.sh` 会下载这样的 release asset：

```text
codeseed-<target>.tar.gz
```

当前 target 名称为：

1. `aarch64-apple-darwin`
2. `x86_64-apple-darwin`
3. `aarch64-unknown-linux-gnu`
4. `x86_64-unknown-linux-gnu`

压缩包中必须包含名为 `codeseed` 的可执行文件。

## 工作流程

1. 确认 release tag 或版本。推荐使用 `v0.1.0` 这类 tag。
2. 检查工作区，避免把无关本地改动打进包里。
3. 发布前运行 `cargo fmt --check` 和 `cargo test`。
4. 构建并打包当前宿主平台：

```bash
presets/skills/codeseed-prebuilt-release/scripts/package-current-target.sh
```

如果这个 skill 已经安装到当前项目，也可以使用等价路径：

```bash
.agent/skills/common/codeseed-prebuilt-release/scripts/package-current-target.sh
```

5. 上传前检查压缩包内容：

```bash
tar -tzf dist/codeseed-<target>.tar.gz
```

6. 上传 asset 到 GitHub release：

```bash
gh release upload <tag> dist/codeseed-<target>.tar.gz --clobber
```

如果 release 还不存在：

```bash
gh release create <tag> dist/codeseed-<target>.tar.gz --title <tag>
```

7. 使用已上传 asset 验证安装路径：

```bash
tmp_home="$(mktemp -d)"
CODESEED_HOME="$tmp_home" CODESEED_INSTALL_MODE=prebuilt ./scripts/install.sh --version <tag> --repo seedware/codeseed
"$tmp_home/bin/codeseed" --version
```

## 汇报要求

汇报已上传的 tag、target、asset 名称和安装验证结果。如果本次只发布了当前宿主平台，也要明确说明。
