# 安装 Codeseed

英文版本：[install.md](install.md)。

Codeseed 默认安装到用户主目录：

```text
~/.codeseed/
├── bin/
│   └── codeseed
├── cache/
└── config/
```

`~/.codeseed/bin/codeseed` 是可执行文件。未来的全局配置，例如默认 SkillHub 设置，可以放在 `~/.codeseed/config/` 下。

## 通过 Shell 脚本安装

```bash
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh | sh
```

安装脚本会检查当前环境：

1. 如果它在 Codeseed 源码仓库中执行，并且本机有 `cargo`，则执行 `cargo build --release`。
2. 否则优先尝试下载 macOS 或 Linux 的预编译二进制。
3. 如果没有可用的预编译二进制，并且本机有 `git` 和 `cargo`，则 clone 仓库并从源码构建。

## 从本地源码安装

在本仓库中执行：

```bash
./scripts/install.sh --local
```

## 更新 Codeseed

使用 CLI 更新命令更新 `codeseed` 可执行文件本身。想先确认安装计划时，先执行 dry run：

```bash
codeseed update --dry-run
codeseed update
```

当命令在 Codeseed 源码仓库中执行时，`codeseed update` 会复用本地 `scripts/install.sh`。否则它会下载并执行配置的安装脚本。默认安装目标仍然是 `~/.codeseed/bin/codeseed`。

也可以直接重新运行 shell 安装脚本：

```bash
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/refs/heads/main/scripts/install.sh | sh
```

更新可执行文件不会自动重写项目内 skills。请在项目根目录单独刷新项目 skills；见 [skill-layout.zh-CN.md](skill-layout.zh-CN.md#刷新项目-skills)。

## 选项

```bash
./scripts/install.sh --version latest
./scripts/install.sh --home "$HOME/.codeseed"
./scripts/install.sh --bin-dir "$HOME/.codeseed/bin"
./scripts/install.sh --prebuilt
```

安装完成后，请确保 `~/.codeseed/bin` 位于 `PATH` 中：

```bash
export PATH="$HOME/.codeseed/bin:$PATH"
```
