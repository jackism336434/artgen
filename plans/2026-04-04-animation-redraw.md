# 改用覆盖重绘减少动画闪烁

## Status
done

## Summary
调整动画刷新策略，去掉每帧对整块区域的清屏操作，改为固定区域覆盖重绘。通过保持每帧输出尺寸一致并补齐行尾空格，减少终端清除与重画之间的闪烁感。

## Key Changes
- 去掉动画循环中的 `Clear(FromCursorDown)`。
- 保留光标回到起始位置，但每帧直接完整覆盖输出。
- 对每行补齐到统一宽度，避免旧帧残留字符。
- README、状态文件和计划文件同步更新。

## Test Plan
- `cargo run -- hello --animate shine --speed 100`
- `cargo run -- hello --animate blink --speed 120`
- `cargo run -- hello --gradient ocean --animate shine`
- `cargo test`

## Assumptions
- ASCII 图案布局在动画过程中保持固定宽度和固定行数。
- 当前优先解决终端刷新闪烁，不引入只更新变化区域的更复杂方案。
