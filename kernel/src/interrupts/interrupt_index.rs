use super::*;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1
}

impl Into<u8> for InterruptIndex {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<usize> for InterruptIndex {
    fn into(self) -> usize {
        self as usize
    }
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self.into()
    }

    pub fn as_usize(self) -> usize {
        self.into()
    }
}