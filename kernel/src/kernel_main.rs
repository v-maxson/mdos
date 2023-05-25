#![allow(
    unused, // Enabled for prototyping.
    special_module_name // For main.rs
)]

// Features
#![feature(
    const_mut_refs, // Mutable refs in const func
    abi_x86_interrupt // x86-interrupts
)]

#![no_std]
#![no_main]

mod main; use main::main;
mod idt;
mod gdt;
mod interrupts;
mod utility;
mod lang_features;

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // Initialize Statics
    unsafe {
        utility::BOOT_INFO.call_once(|| boot_info);

        let boot_info = utility::BOOT_INFO.get_mut_unchecked();
        let info = boot_info.framebuffer.as_ref().unwrap().info();
        let framebuffer = (*boot_info).framebuffer.as_mut().unwrap().buffer_mut();
        utility::FRAME_BUFFER_WRITER.call_once(|| {
            utility::FrameBufferWriter::new(framebuffer, info)
        });
    }

    // Register IDT
    idt::register_idt();

    // Register GDT
    gdt::register_gdt();

    // Initialize PICs
    unsafe { interrupts::PICS.lock().initialize(); }

    // Enable interrupts
    x86_64::instructions::interrupts::enable();

    main();

    loop {}
}

bootloader_api::entry_point!(kernel_main);