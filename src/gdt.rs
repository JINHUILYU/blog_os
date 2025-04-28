use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// 初始化全局描述符表
pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    // 加载 GDT
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector); // 设置代码段，重载 CS 寄存器
        load_tss(GDT.1.tss_selector); // 加载 TSS
    }
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    // 创建一个全局描述符表和选择器
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        // 添加代码段和 TSS 段
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {code_selector, tss_selector})
    };
}

lazy_static! {
    // 创建任务状态段
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // 设置 TSS 的栈指针
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5; // 栈大小
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // 模拟栈存储区
            let stack_start = VirtAddr::from_ptr(unsafe {&raw const STACK});
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}