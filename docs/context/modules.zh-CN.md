# 模块上下文

Rust 源码位于 `src/`。

## 模块地图

1. `src/cli.rs`：clap 命令和参数定义。新增命令形态先改这里。
2. `src/main.rs`：顶层命令分发和面向用户的命令输出。
3. `src/error.rs`：共享的 `CodeseedError` 和 `Result` 类型。
4. `src/init.rs`：`codeseed init`、生成布局、preset 安装 helper、兼容软链接、context 骨架。
5. `src/add.rs`：当前实现 `add preset:<skill-id>`，并更新状态和兼容入口。
6. `src/list.rs`：列出内置 preset skills 或项目已安装 skills。
7. `src/update.rs`：通过安装脚本更新 Codeseed 可执行文件。
8. `src/presets.rs`：preset skill id 和 preset source prefix 常量。
9. `src/remove.rs`：移除已安装的 Codeseed-managed skills 及其生成的兼容入口。
10. `src/clear.rs`：在 dry-run 或确认后清除 Codeseed-managed 项目状态。
11. `src/state.rs`：读取持久 state 字段的小型 helper，不负责完整 state 序列化。
12. `src/lib.rs`：模块导出。

## 命令实现模式

新增命令时：

1. 在 `src/cli.rs` 定义参数。
2. 新增模块，提供 `run(project, command) -> Result<Report>`。
3. 在 `src/main.rs` 接入分发。
4. 在命令模块中添加聚焦单元测试，并在 `src/cli.rs` 添加 clap 解析测试。
5. 更新 [docs/cli.md](../cli.md) 和 [docs/cli.zh-CN.md](../cli.zh-CN.md)。

## 当前缺口

`doctor` 和 `sync` 已有命令面，但尚未完整实现。
