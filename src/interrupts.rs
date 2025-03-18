use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static!(
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // 创建一个新的 IDT
        idt.breakpoint.set_handler_fn(breakpoint_handler); // 设置断点异常的处理函数
        idt
    };
);

pub fn init_idt() {
    IDT.load(); // 加载 IDT
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // 触发断点异常
}
