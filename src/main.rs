#![no_std] // 禁止系统自动链接到标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial; // 导入 serial 模块
mod vga_buffer; // 导入 vga_buffer 模块

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

static HELLO: &[u8] = b"Hello World!";
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    /*
    _start 函数是程序的入口点，这个函数是一个裸函数，没有 Rust 的运行时支持。
     */
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    //
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // // write! 宏类似于 println! 宏，但是它不会自动换行
    // write!(
    //     vga_buffer::WRITER.lock(), // 获取 WRITER 的锁
    //     ", some numbers: {} {}\n",
    //     42,
    //     1.337
    // )
    // .unwrap();
    //
    // print!("Hello again");
    // println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// 定义 panic 函数，这个函数将在出现 panic 时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    /*
    PanicInfo 包含了 panic 的信息，比如 panic 的文件名、行号、panic 的信息等。
    ! 表示这个函数从不返回，这是因为 panic 之后我们无法恢复，只能停止程序。
     */
    if let Some(location) = _info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            _info.message()
        );
    } else {
        println!("{}", _info.message());
    }

    loop {}
}

// panic handle in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success); // 退出 QEMU
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
