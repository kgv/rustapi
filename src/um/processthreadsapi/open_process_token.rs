use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Open process token.
#[derive(FnOnce, TypedBuilder)]
pub struct OpenProcessToken<'a> {
    process_handle: &'a Handle,
    desired_access: u32,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    token_handle: MaybeUninit<Handle>,
}

impl FnOnce<()> for OpenProcessToken<'_> {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::OpenProcessToken;

        #[allow(non_snake_case)]
        unsafe {
            let ProcessHandle = self.process_handle.as_raw_handle();
            let DesiredAccess = self.desired_access;
            let TokenHandle = self.token_handle.as_mut_ptr() as _;
            let r#return = OpenProcessToken(ProcessHandle, DesiredAccess, TokenHandle);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.token_handle.assume_init())
        }
    }
}
