use super::HeapEntry;
use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use partial_uninit::PartialUninit;
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Heap next.
#[derive(FnOnce, TypedBuilder)]
pub struct Heap32ListNext<'a> {
    snapshot: &'a Handle,
    #[builder(default = PartialUninit::partial_uninit(), setter(skip))]
    entry: MaybeUninit<HeapEntry>,
}

impl FnOnce<()> for Heap32ListNext<'_> {
    type Output = Result<HeapEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Heap32ListNext;

        #[allow(non_snake_case)]
        unsafe {
            let hSnapshot = self.snapshot.as_raw_handle();
            let lppe = self.entry.as_mut_ptr() as _;
            let r#return = Heap32ListNext(hSnapshot, lppe);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.entry.assume_init())
        }
    }
}
