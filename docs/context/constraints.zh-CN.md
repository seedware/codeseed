# 约束上下文

修改 Codeseed 时遵循这些约束。

## 文档

1. 面向用户的英文 Markdown 文档应尽量提供中文对应版本。
2. 保持 `docs/context/README.md` 简洁。将细节放入聚焦上下文文件。
3. 命令行为变化时同步更新文档。

## Rust

1. 增加抽象前，优先复用已有模块和 helper。
2. CLI 解析集中在 `src/cli.rs`。
3. 文件系统行为要确定，并谨慎处理用户拥有的文件。
4. 除非命令明确拥有路径或用户请求 force 行为，否则避免破坏性文件系统操作。
5. 提交到仓库中的项目兼容软链接使用相对目标。

## 验证

Rust 修改后运行：

```bash
cargo fmt --check
cargo test
```

CLI 行为变化时，还要运行相关 help 或命令输出，例如：

```bash
cargo run -- list
cargo run -- update --dry-run
cargo run -- init
```

## 生成的 Skill 内容

修改 preset skills 时：

1. 更新 `presets/skills/<skill-id>/`；
2. 当当前仓库也应该使用它时，安装或刷新到 `.agent/skills/common/<skill-id>/`；
3. 验证 `.codeseed/state.json` 和兼容软链接。

