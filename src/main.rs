#![no_std] // 禁止系统自动链接到标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)] // 启用自定义测试框架
#![test_runner(blog_os::test_runner)] // 设置测试框架
#![reexport_test_harness_main = "test_main"] // 测试框架入口函数

use blog_os::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    /*
    _start 函数是程序的入口点，这个函数是一个裸函数，没有 Rust 的运行时支持。
     */
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    println!("Hello World!");

    blog_os::init(); // 初始化 IDT

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // 触发一个页错误
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow(); // 递归调用，导致栈溢出
    // }
    
    // 栈溢出测试
    // stack_overflow();
    
    #[cfg(test)]
    test_main(); // 测试框架入口函数

    println!("It did not crash!");
    blog_os::hlt_loop();
    // loop {
    //     use blog_os::print;
    //     print!("-");
    //     for i in 0..100000{}
    // }
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

    blog_os::hlt_loop();
}

// panic handle in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
