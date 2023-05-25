mod interrupt_index; pub use interrupt_index::*;

use pic8259::ChainedPics;
use spin::Mutex;

// Offsets
pub const PIC_1: u8 = 32;
pub const PIC_2: u8 = PIC_1 + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1, PIC_2) });