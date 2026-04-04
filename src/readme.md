# 项目名称
**彩色终端艺术字生成器（ASCII Art Text Generator）**

# 项目是干什么的？
一句话：
**输入一段文字 → 程序在终端里输出超大号、彩色的 ASCII 艺术字。**

就像这样：
```
██████╗░██████╗░██╗   ██╗██████╗░
██╔══██╗██╔══██╗██║   ██║██╔══██╗
██████╔╝██████╔╝██║   ██║██████╔╝
██╔══██╗██╔══██╗██║   ██║██╔══██╗
██║  ██║██║  ██║╚██████╔╝██║  ██║
╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝
```
当前已经支持**纯色输出**、**多种预设渐变**和**基础动画效果**。

---

# 当前已实现能力
1. **把普通文字转成大号 ASCII 艺术字**
2. **给艺术字上色：纯色 / 预设渐变 / 两端点渐变**
3. **支持未加引号的多词文本输入**
4. **通过命令行选择颜色和动画模式**

当前预设渐变包括：

- `rainbow`
- `sunset`
- `ocean`
- `fire`

当前也支持两端点自定义渐变：

- `--from red --to blue`
- `--from yellow --to magenta`

当前动画能力：

- `--animate blink`
- `--animate shine`
- `--speed <ms>` 控制动画单帧间隔

其中 `shine` 当前采用窄白色扫光：

- 带外区域保持原始颜色不变
- 只有极少数列参与高光移动

当前动画刷新会尽量使用覆盖重绘，减少每帧整块清屏带来的闪烁感。

尚未实现：

- 滚动等更多动画
- hex / RGB 字面量颜色输入
 

---

# 用到的 Rust 技术 & 库


1. **cargo**
  


2. **clap + #[derive(Parser)]**
   接收命令行参数，让你可以：
   自定义文字、颜色、是否开启动画

3. **figlet-rs**
   把普通文字 → 大号艺术字

4. **crossterm**
   给终端文字上色，并为后续动画效果预留终端控制能力



---

# 项目整体流程
1. 用户在终端输入文本和颜色参数
2. **clap** 解析参数，拿到文字和颜色
3. **figlet-rs** 生成艺术字图案
4. 程序按单色或预设渐变给图案**逐字符上色**
5. 最终在终端显示彩色大字


# 当前 CLI 示例

```bash
cargo run -- hello
cargo run -- hello world --color cyan
cargo run -- hello --gradient rainbow
cargo run -- hello --gradient sunset
cargo run -- hello --from red --to blue
cargo run -- hello --animate blink --speed 120
cargo run -- hello --gradient ocean --animate shine --speed 100
```



# 一句话总结
这是一个**用 Rust 写的、能在终端生成彩色 ASCII 艺术字的小工具**，
当前重点是把 CLI、渲染和颜色系统打磨稳定，再继续扩展更丰富的渐变和动画。

