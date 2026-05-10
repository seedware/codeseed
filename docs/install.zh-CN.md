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
curl -fsSL https://raw.githubusercontent.com/seedware/codeseed/main/scripts/install.sh | sh
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

