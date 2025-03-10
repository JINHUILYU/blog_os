#![no_std] // 禁止系统自动链接到标准库
#![no_main] // 禁用所有 Rust 层级的入口点

mod vga_buffer; // 导入 vga_buffer 模块

use core::panic::PanicInfo;
static HELLO: &[u8] = b"Hello World!";
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    /*
    _start 函数是程序的入口点，这个函数是一个裸函数，没有 Rust 的运行时支持。
     */
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write! 宏类似于 println! 宏，但是它不会自动换行
    write!(
        vga_buffer::WRITER.lock(),  // 获取 WRITER 的锁
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}

// 定义 panic 函数，这个函数将在出现 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /*
    PanicInfo 包含了 panic 的信息，比如 panic 的文件名、行号、panic 的信息等。
    ! 表示这个函数从不返回，这是因为 panic 之后我们无法恢复，只能停止程序。
     */
    loop {}
}
