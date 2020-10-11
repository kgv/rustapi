use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;

/// Set thread affinity mask.
#[derive(FnOnce, TypedBuilder)]
pub struct SetThreadAffinityMask<'a> {
    thread: &'a Handle,
    #[builder(default, setter(skip))]
    mask: usize,
}

impl FnOnce<()> for SetThreadAffinityMask<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winbase::SetThreadAffinityMask;

        #[allow(non_snake_case)]
        unsafe {
            let hThread = self.thread.as_raw_handle();
            let dwThreadAffinityMask = self.mask;
            let r#return = SetThreadAffinityMask(hThread, dwThreadAffinityMask);
            ensure!(r#return != 0, io::Error::last_os_error());
            Ok(r#return)
        }
    }
}
