use spin::Once;

type BootInfoMut = &'static mut bootloader_api::BootInfo;
type OnceBootInfoMut = Once<BootInfoMut>;
pub struct WrappedBootInfo(OnceBootInfoMut);

impl WrappedBootInfo {
    pub const fn new() -> Self {
        Self(Once::INIT)
    }
}

unsafe impl Sync for WrappedBootInfo {}
unsafe impl Send for WrappedBootInfo {}

impl core::ops::Deref for WrappedBootInfo {
    type Target = OnceBootInfoMut;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for WrappedBootInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub static mut BOOT_INFO: WrappedBootInfo = WrappedBootInfo::new();