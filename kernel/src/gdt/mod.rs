mod tss; pub use tss::*;

use spin::Lazy;
use x86_64::{structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, registers::segmentation::{CS, Segment}, instructions::tables::load_tss};

pub static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();

    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

    (gdt, Selectors { code_selector, tss_selector })
});

pub struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

pub fn register_gdt() {
    GDT.0.load();

    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}