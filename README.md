# 嵌入式 Rust 学习工作区

板子：STM32 Nucleo-F401RE（主）、STM32F407VET6（第二块）

## lesson1/ —— 第 1 课作业

```bash
cd lesson1
cargo test
```

输出 `test result: ok. 3 passed` 就算过了。代码里三处 `todo!()` 换成你的实现。

## nucleo-blinky/ —— 第 4 周用的点灯工程（已编译验证）

```bash
cd nucleo-blinky
cargo build        # 只编译，不需要板子
cargo run          # 插上板子才用，会用 probe-rs 烧录并打印日志
```

已交叉编译通过，产物是 ARM ELF，加载地址 `0x08000000`，text 段 1.8 KB。

注意：`.cargo/config.toml` 里已经写好默认 target 和 runner，所以不带参数就是交叉编译。

## 环境

| 组件 | 状态 |
|---|---|
| rustc / cargo 1.99.1 | 已装，`~/.zshenv` 已配好 PATH |
| target `thumbv7em-none-eabihf` | 已装（Cortex-M4F 交叉编译用） |
| probe-rs 0.32.0 | 已装（烧录 + 日志 + 断点） |
| rustlings | 见 `rustlings init` |

## 第 4 周插板子前要做的事

1. 确认板子是 **Nucleo-64 F401RE**（板载 ST-Link，一根 USB 就够）
2. 插上后在工程目录跑 `probe-rs list`，应该能看到一个 ST-Link
3. 看不到 → 装 udev 规则（需要 sudo）：

```bash
curl -L https://probe.rs/files/69-probe-rs.rules -o /tmp/69-probe-rs.rules
sudo cp /tmp/69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```
