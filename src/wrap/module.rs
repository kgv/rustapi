use crate::{
    shared::minwindef::ModuleHandle,
    um::libloaderapi::{disable_thread_library_calls, get_module_handle, GetProcAddress},
};
use anyhow::Result;
use derive_more::{From, Into};
use winapi::shared::minwindef::HMODULE;

/// Module.
#[derive(Clone, Copy, From, Into)]
#[repr(transparent)]
pub struct Module(ModuleHandle);

impl Module {
    pub fn new(handle: HMODULE) -> Self {
        Self(ModuleHandle::from(handle))
    }

    pub fn with_name(name: &str) -> Result<Self> {
        get_module_handle().name(name)().map(Self)
    }

    pub fn disable_thread_library_calls(&self) -> Result<()> {
        disable_thread_library_calls().handle(self.0)()
    }

    pub fn function_address(&self, function_name: &str) -> Result<usize> {
        GetProcAddress::builder()
            .handle(&self.0)
            .name(function_name)
            .build()()
    }
}
