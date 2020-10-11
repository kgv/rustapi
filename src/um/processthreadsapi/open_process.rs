use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::FromRawHandle};
use typed_builder::TypedBuilder;
use winapi::um::winnt::PROCESS_ALL_ACCESS;

/// Open process.
#[derive(FnOnce, TypedBuilder)]
pub struct OpenProcess {
    #[builder(default = PROCESS_ALL_ACCESS)]
    desired_access: u32,
    #[builder(default)]
    inherit_handle: bool,
    id: u32,
}

impl FnOnce<()> for OpenProcess {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::OpenProcess;

        #[allow(non_snake_case)]
        unsafe {
            let dwDesiredAccess = self.desired_access;
            let bInheritHandle = self.inherit_handle as _;
            let dwProcessId = self.id;
            let r#return = OpenProcess(dwDesiredAccess, bInheritHandle, dwProcessId);
            ensure!(!r#return.is_null(), io::Error::last_os_error());
            Ok(Handle::from_raw_handle(r#return))
        }
    }
}
