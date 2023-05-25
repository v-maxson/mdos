mod breakpoint; use breakpoint::breakpoint_handler;
mod double_fault; use double_fault::double_fault_handler;
mod timer_interrupt; use timer_interrupt::timer_interrupt_handler;

use crate::interrupts::InterruptIndex;
use x86_64::structures::idt::InterruptDescriptorTable;
use spin::Lazy;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    // Register handlers.
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX); }

    // PIC
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

    idt
});

pub fn register_idt() {
    IDT.load();
}