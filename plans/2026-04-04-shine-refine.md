# 收紧流光效果，去掉 blink 感

## Status
done

## Summary
调整 `shine` 动画的高光算法，使其表现为窄白色扫光，而不是大范围局部提亮。目标是让带外区域完全保持原色，只让极少数列参与白色高光移动，从视觉上彻底区别于 `blink`。

## Key Changes
- 将流光带宽收窄到 1 到 2 列有效高光范围。
- 带外区域完全返回原始颜色，不做任何提亮或变暗。
- 中心列使用更强的白色混合，相邻列使用较弱混合。
- 更新相关测试和状态说明。

## Test Plan
- `cargo run -- hello --animate shine`
- `cargo run -- hello --gradient ocean --animate shine --speed 100`
- `cargo run -- hello --animate blink`
- `cargo test`

## Assumptions
- 第一版修正优先解决“看起来像 blink”的问题。
- 不新增流光宽度或强度参数，先把默认效果调对。
