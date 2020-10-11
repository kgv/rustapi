use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Is wow64 process.
#[derive(FnOnce, TypedBuilder)]
pub struct IsWow64Process<'a> {
    process: &'a Handle,
    #[builder(default = FALSE, setter(skip))]
    wow64_process: i32,
}

impl FnOnce<()> for IsWow64Process<'_> {
    type Output = Result<bool>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::wow64apiset::IsWow64Process;

        #[allow(non_snake_case)]
        unsafe {
            let hProcess = self.process.as_raw_handle();
            let Wow64Process = &mut self.wow64_process;
            let r#return = IsWow64Process(hProcess, Wow64Process);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.wow64_process != FALSE)
        }
    }
}
