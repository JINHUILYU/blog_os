#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use blog_os::{exit_qemu, QemuExitCode, serial_print, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    blog_os::gdt::init(); // 初始化 GDT
    init_test_idt(); // 初始化测试 IDT

    // 触发一个栈溢出异常
    stack_overflow();
    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // 递归调用，导致栈溢出
    volatile::Volatile::new(0).read(); // 防止优化
}

lazy_static! {
    // 创建测试使用的 IDT，便于在测试 double fault 时，调用 exit_qemu
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // 创建一个新的 IDT
        unsafe {
        idt.double_fault
            .set_handler_fn(test_double_fault_handler) // 设置双重故障异常的处理函数
            .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX); // 设置双重故障异常的处理函数
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]\n");
    exit_qemu(QemuExitCode::Success); // 退出 QEMU
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}