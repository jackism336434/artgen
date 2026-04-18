# artgen

`artgen` 是一个用 Rust 编写的终端 ASCII 艺术字生成工具。它可以把输入文本渲染成 FIGlet 风格的大号字符，并为输出应用纯色、渐变和简单动画效果。

## 功能特性

- 将普通文本渲染为终端 ASCII 艺术字
- 支持纯色输出
- 支持预设渐变：`rainbow`、`sunset`、`ocean`、`fire`
- 支持双端点自定义渐变：`--from <color> --to <color>`
- 支持动画效果：`blink`、`shine`
- 支持通过 `--speed <ms>` 控制动画帧间隔
- 支持多词文本输入，不强制要求额外加引号

## 技术栈

- Rust 2024 Edition
- `clap`：命令行参数解析
- `figlet-rs`：ASCII 艺术字渲染
- `crossterm`：终端颜色和光标控制
- `ctrlc`：动画模式下优雅处理中断

## 运行要求

- 支持 ANSI 颜色输出的终端

## 快速开始（直接运行）

下载或获取编译好的可执行文件后，直接运行：

```bash
# Windows
artgen.exe hello

# 或在 PowerShell 中
.\artgen.exe hello --gradient rainbow

# 或将 artgen.exe 加入 PATH 后直接使用
artgen hello --color cyan
```
第二种方法

git clone下来之后，在根目录执行
``` bash
cargo install --path .

```

## 开发运行（需要 Rust）

安装依赖并直接运行：

```bash
cargo run -- hello
```

构建发行版：

```bash
cargo build --release
```

运行测试：

```bash
cargo test
```

## 命令格式

```bash
artgen <TEXT...> [--color <COLOR> | --gradient <GRADIENT> | --from <COLOR> --to <COLOR>] [--animate <ANIMATION>] [--speed <MS>]
```

说明：

- `<TEXT...>`：必填，支持一个或多个单词
- `--color` 与 `--gradient` 互斥
- `--from` 和 `--to` 必须同时使用，且不能与 `--color` 或 `--gradient` 同时使用
- `--alpha-bg` 用于指定 alpha 颜色混合背景，默认是黑色
- `--speed` 只能与 `--animate` 一起使用
- 未指定颜色模式时，默认使用白色纯色输出

## 支持的参数

### 颜色

可用于 `--color`，也可用于 `--from` / `--to`：

- `red`
- `green`
- `blue`
- `yellow`
- `cyan`
- `magenta`
- `white`

也支持字面量颜色：

- `#f60`
- `#f608`
- `#ff6600`
- `#ff660080`
- `#00aaff`
- `#FFFFFF`
- `rgb(255,102,0)`
- `255,102,0`

### 渐变预设

- `rainbow`
- `sunset`
- `ocean`
- `fire`

### 动画

- `blink`：明暗交替闪烁
- `shine`：窄范围高光从左向右扫过

## 使用示例

```bash
# 默认白色输出
artgen.exe hello

# 纯色
artgen.exe hello world --color cyan
artgen.exe hello --color '#f60'
artgen.exe hello --color '#f608'
artgen.exe hello --color '#ff6600'
artgen.exe hello --color '#ff660080'
artgen.exe hello --color '#ff660080' --alpha-bg white
artgen.exe hello --color 'rgb(255,102,0)'
artgen.exe hello --color '255,102,0'

# 预设渐变
artgen.exe hello --gradient rainbow
artgen.exe hello --gradient sunset

# 自定义双端点渐变
artgen.exe hello --from red --to blue
artgen.exe hello --from '#f00' --to '#0af'
artgen.exe hello --from '#f00f' --to '#00aaff80'
artgen.exe hello --from '#f00f' --to '#00aaff80' --alpha-bg '#202020'
artgen.exe hello --from '#ff0000' --to '#00aaff'
artgen.exe hello --from 'rgb(255,0,0)' --to '0,170,255'

# 动画
artgen.exe hello --animate blink --speed 120
artgen.exe hello --gradient ocean --animate shine --speed 100
```

如果使用动画模式，程序会持续运行，按 `Ctrl+C` 可退出。

## 项目结构

```text
.
├─ src/
│  ├─ main.rs      # 程序入口
│  ├─ cli.rs       # CLI 参数定义与校验
│  ├─ render.rs    # ASCII 艺术字渲染
│  └─ color.rs     # 颜色、渐变与动画输出
├─ plans/          # 迭代计划与设计记录
├─ CURRENT_STATUS.md
├─ PROJECT_RULES.md
└─ Cargo.toml
```

## 当前限制

- 当前自定义颜色支持内置命名颜色、`#RGB`、`#RGBA`、`#RRGGBB`、`#RRGGBBAA`、`rgb(r,g,b)` 和 `r,g,b`
- `alpha` 默认按黑色背景做混合，也可以通过 `--alpha-bg <COLOR>` 指定背景色
- 当前不支持百分比 RGB
- 动画效果目前只有 `blink` 和 `shine`
- 输出效果依赖终端对 ANSI 颜色和光标移动的支持

## 当前状态

当前项目处于 CLI 基础能力完善阶段，核心渲染、颜色模式、渐变模式和基础动画已经可用，且现有单元测试已覆盖主要参数解析和颜色逻辑。
