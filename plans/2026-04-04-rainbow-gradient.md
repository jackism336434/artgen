# 增加左到右彩虹渐变输出

## Summary
下一步在现有单色输出基础上，新增一个独立的 `--gradient` CLI 参数，第一版只支持固定名称 `rainbow`。用户可以继续使用 `--color` 做整块单色输出，也可以改用 `--gradient rainbow` 获得从左到右的渐变效果。

这一阶段只做静态渐变，不做动画，不支持自定义起止颜色。

## Key Changes
- 扩展 CLI：
  - 保留现有 `--color <name>`。
  - 新增 `--gradient <name>`，第一版只接受 `rainbow`。
  - `text` 参数保持当前多词输入能力不变。
- 约束颜色模式：
  - `--color` 和 `--gradient` 视为互斥选项，避免同时传入时产生优先级歧义。
  - 两者都不传时，继续默认使用 `white` 单色输出。
- 扩展颜色输出层：
  - 将当前“整块一次设色”的输出逻辑改为支持两种模式：单色模式、渐变模式。
  - 渐变模式按每一列字符的位置计算颜色，从左到右推进颜色变化。
  - 空格和换行仍按原始渲染结果保留；渐变按列位置决定颜色，不按单词或字符重新布局。
- `rainbow` 的实现策略：
  - 使用固定的彩虹调色板，至少包含红、黄、绿、青、蓝、洋红这几个阶段。
  - 对 ASCII 艺术字的可见宽度做归一化映射，把每一列映射到调色板中的一个颜色。
  - 每一行使用同一套列到颜色的映射，保证视觉上是完整的左到右渐变，而不是逐行重新开始。

## Public Interface Changes
- 新增 CLI 参数：
  - `--gradient rainbow`
- 允许的调用方式包括：
  - `cargo run -- hello`
  - `cargo run -- hello --color red`
  - `cargo run -- hello --gradient rainbow`
  - `cargo run -- hello world --gradient rainbow`
- 非法调用规则：
  - 同时传 `--color` 和 `--gradient` 时，参数解析应报错。
  - 传入不支持的渐变名称时，参数解析应报错。

## Test Plan
- `cargo run -- hello --gradient rainbow`
  - 应输出左到右彩虹渐变的大号艺术字。
- `cargo run -- hello world --gradient rainbow`
  - 应支持未加引号的多词文本，并正确渐变。
- `cargo run -- "hello world" --gradient rainbow`
  - 应保持与未加引号输入一致的输出行为。
- `cargo run -- hello --color red`
  - 现有单色输出不回归。
- `cargo run -- hello --color red --gradient rainbow`
  - 应报互斥参数错误。
- `cargo run -- hello --gradient sunset`
  - 应报非法渐变名称错误。
- `cargo run -- "   " --gradient rainbow`
  - 应继续报空白文本错误。

## Assumptions
- 第一版渐变只实现 `rainbow`，不支持用户自定义起止颜色，也不支持多个预设渐变。
- 渐变按列生效，不区分空格是否可见；也就是说，空白列仍参与左右位置计算，这样整块字的颜色过渡更稳定。
- 当前继续使用 `crossterm` 颜色能力，不额外引入新依赖；如果后续需要更平滑的 RGB 渐变，再单独规划升级。
