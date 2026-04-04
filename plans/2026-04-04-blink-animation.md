# 增加第一版闪烁动画基础设施

## Status
done

## Summary
在现有静态颜色输出基础上，新增 `--animate blink`。第一版动画默认持续循环直到用户按 `Ctrl+C` 结束，并复用现有纯色、预设渐变、两端点渐变作为基础颜色模式。

## Key Changes
- 新增 `--animate <name>` CLI 参数，第一版只支持 `blink`。
- 输出层从一次性打印扩展为支持逐帧重绘。
- 闪烁动画对整块 ASCII 艺术字做亮态与暗态切换，不改变原始颜色分布。
- 动画期间隐藏光标，退出时恢复光标与颜色。
- 使用 `Ctrl+C` 作为动画正常结束方式。

## Test Plan
- `cargo run -- hello --animate blink`
- `cargo run -- hello --color cyan --animate blink`
- `cargo run -- hello --gradient sunset --animate blink`
- `cargo run -- hello --from red --to blue --animate blink`
- `cargo run -- hello`
- `cargo test`

## Assumptions
- 第一版只实现 `blink`，不同时实现流光或滚动。
- 暗态采用降低亮度而不是整块隐藏。
- 暂不增加 `--duration`、`--fps`、`--cycles` 等参数。
