use super::ModuleEntry;
use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use partial_uninit::PartialUninit;
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Module next.
#[derive(FnOnce, TypedBuilder)]
pub struct Module32Next<'a> {
    snapshot: &'a Handle,
    #[builder(default = PartialUninit::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ModuleEntry>,
}

impl FnOnce<()> for Module32Next<'_> {
    type Output = Result<ModuleEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Module32NextW;

        #[allow(non_snake_case)]
        unsafe {
            let hSnapshot = self.snapshot.as_raw_handle();
            let lppe = self.entry.as_mut_ptr() as _;
            let r#return = Module32NextW(hSnapshot, lppe);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.entry.assume_init())
        }
    }
}
