//! Nucleo-F401RE 点灯 —— 嵌入式 Rust 的 "Hello, world"。
//!
//! 烧录：`cargo run`（需要 probe-rs，且板子插在 USB 上）
//!
//! 这个文件第 4 课会逐行讲，先记住这几个标记：
//! - `#![no_std]`   没有操作系统、没有 Vec/String/println，只有 core
//! - `#![no_main]`  没有标准库的 main，入口由 cortex-m-rt 提供
//! - `#[entry]`     真正的入口，返回类型 `!` 表示"永不返回"
//! - `Peripherals::take()` 硬件寄存器只有一份，只能取一次 —— 唯一性由类型系统保证

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // panic 就原地死循环，no_std 最小的兜底策略
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // 取走全部外设的所有权。第二次 take() 返回 None —— 这就是 Rust 阻止
    // 两个驱动抢同一个寄存器的方式。
    let dp = pac::Peripherals::take().unwrap();

    // split 需要 &mut RCC，用它打开 GPIOA 的时钟门控。
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);

    // PA5 = Nucleo-64 板上的绿色用户 LED (LD2)
    let mut led = gpioa.pa5.into_push_pull_output();

    // 芯片上电默认就是 HSI 16 MHz，所以这里先不配 PLL（第 5 课再说）。
    // delay 参数是 CPU 周期数：16 MHz 下 2_000_000 ≈ 0.125 秒。
    loop {
        led.set_high();
        cortex_m::asm::delay(2_000_000);
        led.set_low();
        cortex_m::asm::delay(2_000_000);
    }
}
