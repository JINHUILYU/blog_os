#![no_std] // 禁止系统自动链接到标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;


#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    /*
    _start 函数是程序的入口点，这个函数是一个裸函数，没有 Rust 的运行时支持。
     */
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    println!("Hello World!");

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
    blog_os::test_panic_handler(info)
}
