use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static!(
    // 使用 lazy_static 宏来创建一个静态的 IDT 实例
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // 创建一个新的 IDT
        // 在 IDT 中设置断点异常和双重故障异常的处理函数
        idt.breakpoint.set_handler_fn(breakpoint_handler); // 设置断点异常的处理函数
        unsafe {
            // 使用 set_stack_index 实现栈切换，出现故障时切换到安全栈
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // 设置双重故障异常的处理函数
        }
        idt
    };
);

// 初始化中断描述符表
pub fn init_idt() {
    IDT.load(); // 加载 IDT
}

// 处理断点异常的函数
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// 处理双重故障异常的函数
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame); // 打印异常信息并进入 panic 状态
}

// 测试用例，用于测试断点异常处理函数
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // 触发断点异常
}
