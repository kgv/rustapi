use super::ThreadEntry;
use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use partial_uninit::PartialUninit;
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Thread next.
#[derive(FnOnce, TypedBuilder)]
pub struct Thread32Next<'a> {
    snapshot: &'a Handle,
    #[builder(default = PartialUninit::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ThreadEntry>,
}

impl FnOnce<()> for Thread32Next<'_> {
    type Output = Result<ThreadEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Thread32Next;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Thread32Next(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}
