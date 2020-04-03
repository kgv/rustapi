use crate::{
    shared::minwindef::ModuleHandle,
    um::libloaderapi::{DisableThreadLibraryCalls, GetModuleHandle, GetProcAddress},
};
use anyhow::Result;
use derive_more::{From, Into};

/// Module.
#[derive(Clone, Copy, From, Into)]
#[repr(transparent)]
pub struct Module(ModuleHandle);

impl Module {
    pub fn with_name(name: &str) -> Result<Self> {
        GetModuleHandle::builder().module_name(name).build()().map(Self)
    }

    pub fn disable_thread_library_calls(&self) -> Result<()> {
        DisableThreadLibraryCalls::builder().handle(&self.0).build()()
    }

    pub fn function_address(&self, function_name: &str) -> Result<usize> {
        GetProcAddress::builder()
            .handle(&self.0)
            .name(function_name)
            .build()()
    }
}
