use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::FromRawHandle};
use typed_builder::TypedBuilder;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;

/// Create snapshot.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateToolhelp32Snapshot {
    flags: u32,
    #[builder(default)]
    process_id: u32,
}

impl FnOnce<()> for CreateToolhelp32Snapshot {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::CreateToolhelp32Snapshot;

        #[allow(non_snake_case)]
        unsafe {
            let dwFlags = self.flags;
            let th32ProcessID = self.process_id;
            let r#return = CreateToolhelp32Snapshot(dwFlags, th32ProcessID);
            ensure!(r#return != INVALID_HANDLE_VALUE, io::Error::last_os_error());
            Ok(Handle::from_raw_handle(r#return))
        }
    }
}
