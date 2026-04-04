# 增加动画通用时间参数

## Status
done

## Summary
新增 `--speed <ms>` 作为动画通用时间参数，用于控制动画的单帧间隔。第一版先应用于 `blink`，默认值保持为 `150ms`，并把动画时间从输出层硬编码中抽离到通用配置结构里，方便后续流光等动画复用。

## Key Changes
- 新增 `--speed <ms>` CLI 参数，单位为毫秒整数。
- `--speed` 仅在使用 `--animate` 时允许传入。
- 主流程新增动画配置结构，而不是只传裸的动画名称。
- `blink` 动画循环读取配置里的帧间隔，不再写死 `150ms`。
- README、状态文件和计划文件同步更新。

## Test Plan
- `cargo run -- hello --animate blink`
- `cargo run -- hello --animate blink --speed 100`
- `cargo run -- hello --animate blink --speed 200`
- `cargo run -- hello --gradient sunset --animate blink --speed 120`
- `cargo run -- hello --speed 150`
- `cargo run -- hello --animate blink --speed 0`
- `cargo test`

## Assumptions
- `--speed` 表示单帧间隔，不表示完整动画周期。
- 默认值为 `150ms`，以保持当前闪烁观感不变。
- 第一版只让 `blink` 使用该参数，但接口命名按未来动画复用来设计。
