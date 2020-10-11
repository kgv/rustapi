use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Get thread exit code.
#[derive(FnOnce, TypedBuilder)]
pub struct GetExitCodeThread<'a> {
    thread: &'a Handle,
    #[builder(default, setter(skip))]
    exit_code: u32,
}

impl FnOnce<()> for GetExitCodeThread<'_> {
    type Output = Result<u32>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetExitCodeThread;

        #[allow(non_snake_case)]
        unsafe {
            let hThread = self.thread.as_raw_handle();
            let lpExitCode = &mut self.exit_code;
            let r#return = GetExitCodeThread(hThread, lpExitCode);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.exit_code)
        }
    }
}
