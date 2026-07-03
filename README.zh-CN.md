# GIF Maker

[English](README.md)

GIF Maker 是一个用 Rust 编写的终端 GIF 生成工具。它通过 Ratatui 提供 TUI 界面，从本地图片帧目录读取文件，并生成循环播放的 GIF。

## 功能

- 默认从 `./frames` 读取图片帧
- 可以在终端界面中选择自定义输入目录
- 支持 `png`、`jpg` 和 `jpeg` 输入文件
- 按文件路径排序后再进行编码
- 将所有帧缩放到第一张图片的尺寸
- 默认输出无限循环播放的 GIF 到 `./output.gif`
- 可以在终端界面中调整 FPS

## 环境要求

- 支持 Rust 2024 edition 的 Rust 工具链
- 一个按可排序文件名组织的图片帧目录，例如：

```text
frames/
  frame_001.png
  frame_002.png
  frame_003.png
```

建议使用带补零的文件名，这样排序后的文件顺序才会符合预期动画顺序。

## 使用方式

创建默认输入目录并放入图片帧：

```sh
mkdir -p frames
cp /path/to/images/* frames/
```

启动应用：

```sh
cargo run
```

TUI 快捷键：

| 按键 | 操作 |
| --- | --- |
| `i` | 编辑输入目录 |
| `Up` | 提高 FPS |
| `Down` | 降低 FPS |
| `Enter` | 生成 GIF |
| `q` | 退出 |

编辑输入目录时：

| 按键 | 操作 |
| --- | --- |
| 文本输入 | 输入目录路径 |
| `Backspace` | 删除前一个字符 |
| `Enter` | 保存输入目录 |
| `Esc` | 取消编辑 |

生成后的 GIF 会写入：

```text
./output.gif
```

## 实现流程

应用启动后会进入 Crossterm 的备用屏幕，由 Ratatui 渲染终端界面，并在事件循环中处理键盘输入。

GIF 生成流程如下：

1. 扫描当前选择的输入目录中支持的图片文件。
2. 对图片路径进行排序。
3. 解码第一张图片，确定 GIF 的宽高。
4. 将每一帧缩放到相同尺寸。
5. 根据当前 FPS 将帧编码为循环播放的 GIF。

## 项目结构

```text
src/
  app.rs           应用状态和 FPS 控制
  file_scanner.rs  图片扫描和排序
  gif_encoder.rs   GIF 编码流程
  main.rs          终端初始化和事件循环
  tui.rs           Ratatui 界面渲染
```

## 开发

运行本地检查：

```sh
cargo check --all-targets --all-features
```

格式化代码：

```sh
cargo fmt
```

运行 Clippy：

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

仓库中也包含 Lefthook 配置，用于提交前检查。更多设置方式请查看 [CONTRIBUTING.md](CONTRIBUTING.md)。
