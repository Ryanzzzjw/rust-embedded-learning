# 嵌入式 Rust 学习计划

**基准假设**：每天 1 小时，每周 6 天（第 7 天是缓冲/复盘，不算进度）。总共约 10 周 ≈ 60 小时。
板子：Nucleo-F401RE（主，板载 ST-Link）；F407VET6 留到第 9 周。

**怎么缩放**：每天只有 30 分钟 → 周数 ×2；每天 2 小时 → 每 2 天吃一天的内容，周数 ×0.5。
别的都不用改，尤其是「验收」那几行——那是判断能不能往下走的唯一标准。

**每天的节奏**（60 分钟）：
15 分钟 复习昨天 + 打开昨天的代码跑一遍 · 35 分钟 新内容 · 10 分钟 写三行日志（今天学会了什么 / 哪个报错没懂）

**为什么前两周按天、后面按周**：近的按天是真排的；五周以后按天排是编的，等你到那儿实际情况一定不一样。

---

## 总览

| 周 | 主题 | 产出 | 验收 |
|---|---|---|---|
| 0 | 环境确认 | — | 四条工具链命令有输出 + `lesson1` 报 3 failed |
| 1 | Rust 地基：变量/类型/函数/控制流/Option/所有权 | `lesson1` 三道题 | `cargo test` → 3 passed |
| 2 | Rust 结构：struct/enum/match/集合/trait/Result | `lesson2` 小项目 | `cargo test` 全绿 |
| 3 | `no_std` 心智 + 位操作 + embedded-hal 阅读 | `lesson3` 无标准库库 | 交叉编译 + host 测试都过 |
| 4 | **上板**：点灯 + probe-rs + defmt + 断点 | 灯按你的频率闪 | `cargo run` 能看到灯闪和日志 |
| 5 | 按键 + 中断 + 消抖 + 定时器 | 按键切灯，不抖 | 连按 20 下不错乱 |
| 6 | 串口 + `core::fmt` + 命令协议 | 串口输入 `led on` 生效 | 电脑上敲命令灯响应 |
| 7 | ADC + I2C 传感器（+ SPI 概念） | 串口打出传感器数值 | 数值随环境变化 |
| 8 | 综合项目 + 复盘 | 串口控制 + 传感器 + 日志 | 断电重插仍工作 |
| 9–10 | RTIC 或 embassy 二选一；F407 迁移 | 最终项目 | 能讲清每个中断的时序 |

累计：W1–2 约 12 h，W3–4 约 12 h，W5–8 约 24 h，W9–10 约 12 h。

---

## 第 0 晚（30 分钟，今晚）

这一步是让你自己确认一遍，别信文档信命令。**先重开一个终端**（装完 PATH 才生效），然后在仓库根目录跑：

```bash
rustc --version                     # 要有版本号
rustup target list --installed      # 要看到 thumbv7em-none-eabihf
probe-rs --version                  # 要有版本号
rustlings --version                 # 要有版本号
cd lesson1
cargo test                          # 预期 3 failed（todo!() 还没填）
```

四条工具链命令都有输出、最后一条是 `3 failed` → 直接进第 1 天。任何一条不对就是把完整报错贴给我。

版本号不用对表：只要命令有输出就说明装对了，具体版本各平台各时间装出来不一样（这个仓库只要求 Rust **1.85+**，因为用了 edition 2024）。真正卡人的是这几句报错：

| 报错 | 原因 | 怎么办 |
|---|---|---|
| `command not found: cargo`（Win: 不是内部或外部命令） | 没重开终端 | 关掉重开；还不行就看 PATH 里有没有 cargo 的 bin 目录 |
| `linker 'cc' not found`（macOS） | 没装 Xcode CLT | `xcode-select --install` |
| `linker 'link.exe' not found`（Windows） | 缺 C++ 生成工具 | 装 Visual Studio Build Tools 的「C++ 桌面开发」 |
| `can't find crate for 'core'` / `target not installed` | 没装 M4F 目标 | `rustup target add thumbv7em-none-eabihf` |

macOS / Linux / Windows 的完整安装步骤（含驱动差异）在 [README.md](README.md#环境安装)。

---

## 第 1 周：Rust 地基（7 天）

目标只有一个：**让编译器当老师**，把报错读到不烦。

下表第四列的 rustlings 练习在仓库里的 `rustlings/` 目录（整个目录被 `.gitignore` 忽略，不参与版本控制）。

| 天 | 内容 | rustlings 对应 | 当天交付 |
|---|---|---|---|
| 1 | 变量默认不可变、`mut`、整数宽度、函数最后一个表达式 | `variables` `functions` | `lesson1` 练习 1 变绿 |
| 2 | `if`/`for`/`while`/`loop`，`0..5` vs `0..=5` | `if` | 练习 2 变绿 |
| 3 | `Option`、`match`、`unwrap_or` | `options` | 练习 3 变绿 → **`cargo test` 3 passed** |
| 4 | 所有权：move 与 borrow；亲手制造 3 个借用报错 | `move_semantics` | 日志里写下 3 个报错原文＋你的翻译 |
| 5 | `&str` vs `String`；`&T` 与 `&mut T` 互斥规则 | `strings` | 写一个函数，参数 `&mut String` 追加内容并证明调用后原值变了 |
| 6 | 复习 + 重做昨天卡住的题 | — | 不查资料写出练习 3 |
| 7 | 缓冲：补欠账或休息 | — | — |

**本周常见坑**（提前知道能省两晚）：
- 忘记 `mut` → `cannot assign twice to immutable variable`
- 函数末尾多写分号 → `expected i32, found ()`。分号＝丢弃返回值。
- `String` 传进函数后原变量不能用了 → 那是 move，不是 bug。要么传 `&`，要么 `.clone()`。

**本周末的自我判断**：能对着一段 20 行的 Rust 代码说出每一行在干什么，且看到 `borrow of moved value` 不再慌 → 可以进第 2 周。

---

## 第 2 周：Rust 结构（7 天，周末做小项目）

这周结束时你手里要有 `lesson2`——一个**故意写成嵌入式形状**的主机程序：模拟寄存器 + 无堆环形缓冲 + 状态机。第 6 周它会原样搬上板子。

| 天 | 内容 | rustlings 对应 | 当天交付 |
|---|---|---|---|
| 1 | `struct` + `impl`；`u32` 位操作（`<<` `\|` `&` `!`） | `structs` | `Register { value: u32 }` 带 `set/clear/toggle/read_bit` |
| 2 | `enum` + `match` 穷尽性 | `enums` | 三状态机 `Idle/Active/Fault`，`Fault` 必须带原因 |
| 3 | `Vec`、`HashMap`、迭代器 `.iter().filter().map().collect()` | `vecs` `hashmaps` `iterators` | 把状态迁移历史存 `Vec` 并按条件统计 |
| 4 | `trait`：定义 + 实现 + `impl Trait` 参数 | `traits` | 定义 `Led` trait，两个假实现；再去 docs.rs 看 `embedded-hal` 的 `OutputPin` 签名 |
| 5 | `Result` + `?` + 自定义错误 + 模块拆分 | `error_handling` `modules` | 解析 `"0x4002_0000"` 成 `u32`，失败返回自己的错误类型而不是 panic |
| 6–7 | 小项目 `lesson2`（可拆两天） | — | `cargo test` 全绿，且 `cargo clippy` 无 warning |

`lesson2` 的四个必做件（都是上板后要用的形状）：
1. `Register`：按手册的「读改写」语义操作单个 bit，不许用 `unsafe`
2. `RingBuffer`：定长数组实现，满了返回 `Err` 而不是扩容（**板上没有堆**）
3. 状态机：`enum` + `match`，非法转移返回 `Err`
4. 测试：至少 8 个 `#[test]`，覆盖边界（空缓冲、满缓冲、非法转移）

**本周末的自我判断**：能说清「为什么这里用 `&mut self` 不用 `&self`」，能解释 `?` 展开成什么 → 可以进第 3 周。

---

## 第 3 周：`no_std` 心智（不上板）

这周不写业务逻辑，只做一件事：搞清楚**没有操作系统时你手里还剩什么**。`Vec`/`String`/`println!`/`std::time`/堆分配全部消失，`panic` 要你自己定义行为。

| 天 | 内容 | 交付 |
|---|---|---|
| 1 | `#![no_std]` `#![no_main]` `#[entry]` 各是干什么的；`panic-halt` 为什么必需 | 用 `rust-src` 打开 `core` 的 `option.rs` 读 20 行源码 |
| 2 | 内存布局：`memory.x` 里 FLASH/RAM 从哪来；`link.x` 干什么 | 手画一张 F401RE 内存图（512K flash / 96K RAM）并解释 `.data` `.bss` `.text` |
| 3 | `volatile` 与 `read_volatile`：为什么编译器会把轮询循环优化没 | 写出「错误版」和「volatile 版」轮询，说明差别 |
| 4 | 位操作进阶：掩码、移位、`bitflags` 风格寄存器块 | 手写一个 `mod reg { pub const CTRL: *mut u32 = ...; }` 的读写封装 |
| 5 | `embedded-hal` 的 trait 地图：`OutputPin` `DelayNs` `SpiDevice` `I2c` `Serial` | 抄一遍 `OutputPin` 的三个方法签名，说清 `ErrorType` 为什么存在 |
| 6 | 层：PAC（寄存器）→ HAL（外设）→ BSP（板子）→ 你的代码 | 在 `stm32f4xx-hal` 文档里指出 `pa5` 从哪来 |
| 7 | 复盘：把第 2 周的 `RingBuffer` 改成 `no_std` 版 | 见下方验收 |

本周产出 `lesson3/`：一个 `#![no_std]` 的库 crate（不是 bin），包含无堆环形缓冲 + 寄存器位操作，`#[cfg(test)]` 在 host 上测。

**验收（两条都要过）**：
```bash
cd lesson3
cargo test                                     # host 上逻辑正确
cargo build --target thumbv7em-none-eabihf     # 嵌入式目标编得过
```
`cargo build` 报 `duplicate lang item` 说明你混进了 `std`；报 `no global memory allocator` 说明你用了 `Vec`——这两个报错就是本周的全部意义。

---

## 第 4 周：上板（第一次摸硬件）

```bash
probe-rs list                     # 插上板子，应该看到一个 ST-Link
cd nucleo-blinky
cargo run                         # 已经编译验证过，这条命令应该直接点亮 LD2
```

看不到 ST-Link → 按平台处理：**Linux** 装 udev 规则、**Windows** 装 ST-Link 驱动、**macOS** 免驱（先查线）。三种情况的步骤在 [README](README.md#第-4-周插板子前要做的事) 那一节。

| 天 | 内容 | 交付 |
|---|---|---|
| 1 | 插板、`probe-rs list`、跑通 blinky；拆开看 `main.rs` 每一行 | 灯闪 |
| 2 | 改频率：`cortex_m::asm::delay()` 的数值和实际周期对不上，自己标定 | 灯 2 Hz，肉眼可数 |
| 3 | 读原理图/datasheet：确认 PA5 = LD2、PC13 = 用户按键（低有效） | 能说出 LED 是灌电流还是拉电流 |
| 4 | `defmt` + RTT：`defmt::info!` 在 `probe-rs run` 里打印 | 屏幕上出现你的日志 |
| 5 | 断点与变量查看：`probe-rs debug`，在循环里停住看变量 | 能停在 `main` 里读到一个值 |
| 6 | 把 blinky 改成「呼吸灯」（软件 PWM，`for` 循环改占空比） | 亮度渐变 |
| 7 | 复盘：写下 3 个「上板才知道」的事实 | — |

本周最容易崩的两处：**烧不进去**（99% 是驱动/线/板子选错芯片型号 —— Linux 查 udev、Windows 查 ST-Link 驱动、macOS 查线）和**烧进去但没反应**（99% 是时钟没使能或引脚选错）。这两个都先看 `probe-rs run` 的日志，别瞎改代码。

---

## 第 5 周：输入、中断、定时器

- 按键 PC13 轮询读 → 再改成 EXTI 中断回调
- 消抖：先 `delay` 傻等（能跑）→ 再改成时间戳判定（正确）
- `TIM2` 定时中断：用中断驱动闪灯，主循环空转
- **验收**：连按 20 下，灯状态切换次数正好 20；按下不松手不会连续触发

中断里只做最少的事（置标志位），逻辑放主循环——这是嵌入式第一条纪律。

## 第 6 周：串口 + 协议

- `USART2`（PA2/PA3，连板载 ST-Link 的虚拟串口），`core::fmt::Write` 实现 `write_str`
- 收命令行：`led on` / `led off` / `status` / `freq 5`
- 把第 2 周的状态机搬上来，命令只是状态转移的输入
- **验收**：电脑串口终端敲 `freq 5`，灯变 5 Hz；敲错命令返回错误而不是死机

`no_std` 里没有 `format!`，先手写 `write!` 到缓冲，别急着上 `alloc`。

## 第 7 周：模拟量与传感器

- ADC：读内部温度或外部电位器，串口打印 mV
- I2C：接一个 BME280 或 MPU6050，读 ID 寄存器确认通，再读数据
- SPI 只读概念：和 I2C 的区别、片选、全双工
- **验收**：串口每秒一行传感器数据，I2C 断线时程序不死（错误走 `Result` 返回）

## 第 8 周：综合项目

把前四周拼成一个东西：串口命令 + 状态机 + 传感器阈值告警 + `defmt` 日志 + 出错自恢复。
**验收**：拔电重插后自己跑起来；故意拔掉传感器线，程序报告错误并继续工作。

## 第 9–10 周：选一条路

- **RTIC**：抢占式调度、资源在编译期分配，适合「中断多、时序硬」的场景。学 2 天能上手。
- **embassy**：`async`/`await`，适合「等外设」多的场景。概念新，但写起来最像 Python。

二选一，别都学。然后用 F407VET6 迁移一次：换芯片换时钟树，体会 PAC/HAL 的分层到底帮你省了什么。
最后做一个小项目收尾（数据采集器 / 电机控制 / USB HID 都行），做完就算入门。

---

## 三条纪律

1. **卡 20 分钟** → `rustlings hint` 或看报错原文；**卡 40 分钟** → 把完整报错贴给我，别硬扛。
2. **每周末跑一遍当周验收命令**，不看答案能写出来才往下走。过不了就再来一周，不丢人——嵌入式的时间都花在这上面。
3. **写日志**。三行就够，但「今天哪个报错没懂」这一栏会在第 8 周救你。

## 明确跳过（很久都不用）

生命周期标注（能读懂就行，不用会写）· 泛型约束的复杂写法 · 宏编写 · `Rc`/`RefCell`/智能指针 · 线程与锁 · `async` 直到第 9 周 · 测试框架（`cargo test` 够用）· 深度优化与 `unsafe`。

这些不是不重要，是**现在学它们不会让你早点点灯**。

## 进度

- [ ] W0 环境
- [ ] W1 lesson1 → 3 passed
- [ ] W2 lesson2 → clippy 干净
- [ ] W3 lesson3 → 交叉编译 + host 测试
- [ ] W4 上板点亮
- [ ] W5 按键中断
- [ ] W6 串口协议
- [ ] W7 传感器
- [ ] W8 综合项目
- [ ] W9–10 RTIC/embassy + F407
