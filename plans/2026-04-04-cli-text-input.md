# 支持未加引号的多词文本输入

## Summary
调整 CLI 的 `text` 参数定义，让程序可以接收多个位置参数，并在内部用空格拼接成最终文本。这样用户既可以输入 `cargo run -- hello`，也可以直接输入 `cargo run -- hello world`，不再强制要求带引号。

## Key Changes
- 将当前单个 `String` 文本参数改为多个文本片段的集合类型，例如 `Vec<String>`。
- 保持 `--color <name>` 接口不变，避免影响现有颜色功能。
- 在主流程里将多个文本片段按单个空格拼接，再交给现有渲染逻辑。
- 继续保留空白输入校验：
  - 没有提供任何文本片段时，交给 `clap` 报参数缺失。
  - 提供的片段拼接后若全为空白，程序继续报错。
- 不改变已有带引号输入的兼容性：
  - `"hello world"` 仍然可用。
  - `hello world` 也将可用。

## Public Interface Changes
- CLI 位置参数 `text` 的语义从“单个字符串”变成“一个或多个文本片段”。
- 用户可接受的调用方式变为：
  - `cargo run -- hello`
  - `cargo run -- hello world`
  - `cargo run -- "hello world"`
  - `cargo run -- hello brave new world --color cyan`

## Test Plan
- `cargo run -- hello`
- `cargo run -- hello world`
- `cargo run -- "hello world"`
- `cargo run -- hello world --color red`
- `cargo run -- --color red`
  - 应报缺少文本参数
- `cargo run -- "   "`
  - 应报空白文本错误
- `cargo run -- hello --color orange`
  - 应继续报非法颜色错误

## Assumptions
- 文本片段之间统一用单个空格拼接，不保留用户输入时可能存在的多个连续空格。
- 当前只解决“未加引号的多词输入”问题，不引入更复杂的转义、分隔符或原始命令行重建逻辑。
