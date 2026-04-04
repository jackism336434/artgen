# 扩展预设渐变并同步当前文档

## Status
done

## Summary
在现有 `--gradient rainbow` 的基础上，把渐变能力扩展为多个固定预设，让 CLI 的颜色模式从“单一特例”变成“可继续扩展的能力”。这一轮继续保持 `--gradient <name>` 接口，不引入动画，也不引入用户自定义起止色。

## Key Changes
- 扩展 `GradientName`，新增少量固定预设渐变名称。
- 把当前仅适用于 `rainbow` 的配色逻辑重构为通用的渐变调色板映射。
- 保持 `--color` 与 `--gradient` 互斥。
- 保持无颜色模式参数时默认白色单色输出。
- 更新文档，使说明与当前 CLI 能力一致，不再把尚未实现的动画描述成已完成功能。
- 补充基础测试，覆盖新增渐变和关键 CLI 约束。

## Test Plan
- `cargo run -- hello --gradient rainbow`
- `cargo run -- hello --gradient sunset`
- `cargo run -- hello world --gradient ocean`
- `cargo run -- hello --gradient fire`
- `cargo run -- hello --color red`
- `cargo run -- hello --color red --gradient rainbow`
- `cargo run -- hello --gradient unknown`
- `cargo test`

## Assumptions
- 本轮新增的预设渐变控制在少量几个，优先建立可扩展结构。
- 渐变继续按 ASCII 艺术字的列位置计算，不按每行重新开始。
- 动画效果仍然留到后续阶段单独规划。
