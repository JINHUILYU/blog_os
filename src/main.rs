#![no_std]
#![no_main]

mod vga_buffer;  // 引入 vga_buffer 模块

use core::panic::PanicInfo;

// panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 打印完整的 PanicInfo
    println!("{info}");
    println!("Panic!");
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

// 裸机入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}
