# 增加窄白色扫光流光动画

## Status
done

## Summary
在现有 `blink` 基础上新增 `shine` 动画。第一版流光采用固定左到右移动的窄白色高光带，带外保持原始颜色不变，继续复用 `--animate` 和 `--speed`。

## Key Changes
- 在 `AnimationName` 中新增 `shine`。
- 输出层新增 `shine` 动画分支，复用现有逐帧重绘、光标恢复和 `Ctrl+C` 退出逻辑。
- 流光带中心强烈向白色混合，边缘快速衰减，带外完全保持原始颜色。
- `--speed` 继续作为动画单帧间隔，同时应用于 `shine`。
- README、状态文件和计划文件同步更新到实际能力。

## Test Plan
- `cargo run -- hello --animate shine`
- `cargo run -- hello --color cyan --animate shine`
- `cargo run -- hello --gradient sunset --animate shine --speed 100`
- `cargo run -- hello --from red --to blue --animate shine --speed 120`
- `cargo run -- hello --animate blink`
- `cargo test`

## Assumptions
- 第一版方向固定为左到右。
- 第一版不增加宽度、强度、方向等额外参数。
- 流光带采用窄白色扫光，而不是大范围柔和提亮。
