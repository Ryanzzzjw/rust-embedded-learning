# 嵌入式 Rust 学习工作区

板子：STM32 Nucleo-F401RE（主）、STM32F407VET6（第二块）

完整计划见 [SCHEDULE.md](SCHEDULE.md)：10 周、每天 1 小时、每周有可验证的验收命令。下面是仓库里各目录对应的进度。

日常用法三个平台完全一样（`cargo build` / `cargo run` / `cargo test` / `rustlings`）；不一样的地方只有**安装**和**装驱动**，这两步下面分开写了。

## lesson1/ —— 第 1 课作业

```bash
cd lesson1
cargo test
```

输出 `test result: ok. 3 passed` 就算过了。代码里三处 `todo!()` 换成你的实现。

`lesson2/`（第 2 周）、`lesson3/`（第 3 周）还没生成，到那一周再建 —— 提前建好只是空壳，反而增加要维护的东西。

## nucleo-blinky/ —— 第 4 周用的点灯工程

```bash
cd nucleo-blinky
cargo build        # 只编译，不需要板子
cargo run          # 插上板子才用，会用 probe-rs 烧录并打印日志
```

产物是 ARM ELF，加载地址 `0x08000000`。实测占 flash：debug 构建 1852 B、release 构建 688 B —— 512 KB flash 用了不到 0.4%，剩下的都是你的。

注意：`.cargo/config.toml` 里已经写好默认 target 和 runner，所以不带参数就是交叉编译 —— 三个平台都这样，Windows 上也不用额外改。

## 环境安装

四样东西：Rust 工具链、Cortex-M4F 目标、probe-rs、rustlings，另外各平台还有一点前置依赖（Windows 的 C++ 生成工具、Linux 的 udev 规则、macOS 的 Xcode CLT）。

Rust 版本要求 **1.85 或更新**（`lesson1` 和 `rustlings` 用了 `edition = "2024"`，这个 edition 从 1.85 开始支持）。装 stable 就行，现在拿到的都满足。装完在任意目录跑一遍自检，全部有输出就到位了：

```bash
rustc --version
rustup target list --installed        # 要看到 thumbv7em-none-eabihf
probe-rs --version
rustlings --version
```

### 1. Rust 工具链（rustup）

**macOS / Linux**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

**Windows**

从 https://rustup.rs 下载 `rustup-init.exe` 运行，一路默认即可（默认装 `x86_64-pc-windows-msvc`）。

Windows 上要先装 C++ 生成工具，缺了会在第一次 `cargo build` 才报错，先装好省事：

- **Visual Studio Build Tools**，勾选「使用 C++ 的桌面开发」工作负载（提供 `link.exe` 和 Windows SDK）。没装会报 `linker 'link.exe' not found`。
- 不想装 VS 也可以切 GNU 工具链（`rustup set default-host x86_64-pc-windows-gnu`），但要另外装 MSYS2/MinGW。后面 probe-rs 用官方预编译包更顺，推荐还是装 Build Tools。

macOS 如果没装 Xcode Command Line Tools，会报 `linker 'cc' not found` —— 跑一次 `xcode-select --install`。

PATH 三个平台的安装器都会自动配好（Windows 写进用户环境变量 `%USERPROFILE%\.cargo\bin`，macOS/Linux 追加到 `~/.cargo/env` 并引入 shell 配置）。**装完必须重开一个终端**，当前终端里 `cargo` 还是找不到的。

### 2. Cortex-M4F 交叉编译目标

```bash
rustup target add thumbv7em-none-eabihf
```

三个平台完全一样。

不需要额外装 `arm-none-eabi-gcc` 或 `gcc-arm-none-eabi` —— Rust 自带 `rust-lld` 链接器，`cortex-m-rt` 也只要求这个。网上很多老教程让你装工具链，是为了 C 或 C++ 混编，这个仓库用不上。

### 3. probe-rs（烧录 / 日志 / 断点）

**macOS / Linux**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

**Windows（PowerShell）**

```powershell
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

**任意平台，走 cargo（要从源码编译，几分钟）**

```bash
cargo install probe-rs-tools --locked
```

Linux 上从源码编译需要先装系统依赖：

```bash
sudo apt install -y pkg-config libudev-dev cmake git      # Debian / Ubuntu
sudo dnf install libusbx-devel libudev-devel cmake git    # Fedora / CentOS
```

macOS 也可以用 Homebrew：`brew install probe-rs-tools`。

### 4. rustlings

```bash
cargo install rustlings
```

三个平台一样。练习目录已经生成在 `rustlings/`，不需要再跑 `rustlings init`。

它是第三方练习项目，有自己独立的 `Cargo.toml` 和编译产物（`target/` 有 280M+）。**整个目录写进 `.gitignore`，不纳入本仓库的版本控制** —— 所以 `git status` 里看不到它，你改练习也不会污染本仓库的提交。想重新生成就 `rm -rf rustlings && rustlings init`。

## rustlings 练习

```bash
cd rustlings
rustlings            # 进入 watch 模式，改文件自动判题
rustlings hint       # 卡住了就抽提示（不直接给答案）
rustlings run if1    # 只跑某一题
```

第 1 周表格里的 `variables` / `functions` / `if` / `options` / `move_semantics` / `strings`
都在这套练习里，改了文件存盘就会自动判。

注意 rustlings 6.x 去掉了 `rustlings list`，网上旧教程里写 `list` 会报错，用 `rustlings check-all` 看总进度。

## 第 4 周插板子前要做的事

**驱动这一步三个平台不一样，别抄错：**

| 平台 | 要做什么 |
|---|---|
| macOS | 什么都不用装，ST-Link 免驱 |
| Linux | 装 udev 规则，否则非 root 用户的 `probe-rs list` 看不到设备 |
| Windows | 装 ST 官方 ST-Link 驱动 **STSW-LINK009**（ST 官网搜这个编号），否则设备管理器里是个带感叹号的未知设备。Win10/11 有时会自动装好，先在设备管理器确认 |

装了 STSW-LINK009 之后 `probe-rs list` 还是空的，先按顺序排查：换数据线 → 换 USB 口 → 另一台电脑。**不要**急着用 Zadig —— 它是把设备改成通用 WinUSB 驱动的工具，装在 ST-Link 上会顶掉官方驱动，之后官方工具和固件升级都用不了了。Zadig 是留给 CMSIS-DAP / J-Link / FTDI 那类调试器的。

Linux 的 udev 规则：

```bash
curl -L https://probe.rs/files/69-probe-rs.rules -o /tmp/69-probe-rs.rules
sudo cp /tmp/69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

在 WSL 里还要让 udev 服务起来：`/etc/wsl.conf` 加

```ini
[boot]
command="service udev start"
```

插上板子后：

1. 确认板子是 **Nucleo-64 F401RE**（板载 ST-Link，一根 USB 就够）
2. 在 `nucleo-blinky/` 里跑 `probe-rs list`，应该能看到一个 ST-Link
3. 看不到 → 用 `probe-rs info` 看是不是认到了但连接失败；再不行换根数据线（充电线不行）、换个 USB 口

ST-Link 固件太旧会直接报错，要求 V2 ≥ 2.26 / V3 ≥ 3.2，用 ST 官网的 ST-Link 固件升级工具 **STSW-LINK007** 升级。

板子不在手边时 `cargo run` 会停在 `Error: No connected probes were found.` —— 这是正常的，说明 runner 已经正确调用 probe-rs，只差硬件。
