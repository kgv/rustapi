use super::ProcessEntry;
use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use partial_uninit::PartialUninit;
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Process first.
#[derive(FnOnce, TypedBuilder)]
pub struct Process32First<'a> {
    snapshot: &'a Handle,
    #[builder(default = PartialUninit::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ProcessEntry>,
}

impl FnOnce<()> for Process32First<'_> {
    type Output = Result<ProcessEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Process32FirstW;

        #[allow(non_snake_case)]
        unsafe {
            let hSnapshot = self.snapshot.as_raw_handle();
            let lppe = self.entry.as_mut_ptr() as _;
            let r#return = Process32FirstW(hSnapshot, lppe);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.entry.assume_init())
        }
    }
}
