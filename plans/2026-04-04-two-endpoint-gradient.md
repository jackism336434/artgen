# 增加两端点自定义渐变

## Status
done

## Summary
在现有预设渐变基础上，新增 `--from <color> --to <color>` 两端点渐变能力。第一版只允许使用现有命名色作为端点，并通过平滑 RGB 插值实现从左到右的颜色过渡。

## Key Changes
- 保留 `--color <name>` 和 `--gradient <name>` 不变。
- 新增 `--from <color>` 与 `--to <color>` 参数，必须成对出现。
- `--from/--to` 与 `--color`、`--gradient` 互斥。
- 颜色层新增命名色到 RGB 的映射，并实现两端点之间的线性插值。
- README 和当前状态文件同步更新当前 CLI 能力。

## Test Plan
- `cargo run -- hello --from red --to blue`
- `cargo run -- hello world --from cyan --to magenta`
- `cargo run -- hello --gradient rainbow`
- `cargo run -- hello --color red`
- `cargo run -- hello --from red`
- `cargo run -- hello --to blue`
- `cargo run -- hello --from red --to blue --color white`
- `cargo run -- hello --from red --to blue --gradient fire`
- `cargo test`

## Assumptions
- 第一版两端点渐变只接受现有 `ColorName` 颜色枚举。
- 渐变按 ASCII 艺术字整块宽度从左到右插值，不按每行重新起算。
- 当前不支持 hex、RGB 字面量或更复杂的插值模型。
