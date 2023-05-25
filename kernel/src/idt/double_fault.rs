use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT ({code})\n{stack_frame:#?}");
}