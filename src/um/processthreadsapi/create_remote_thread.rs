use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{
    io,
    os::windows::io::{AsRawHandle, FromRawHandle},
    ptr::null_mut,
};
use typed_builder::TypedBuilder;
use winapi::um::minwinbase::{LPTHREAD_START_ROUTINE, SECURITY_ATTRIBUTES};

/// Create remote thread.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateRemoteThread<'a> {
    process: &'a Handle,
    #[builder(setter(strip_option))]
    thread_attributes: Option<&'a mut SECURITY_ATTRIBUTES>,
    #[builder(setter(strip_option))]
    stack_size: Option<usize>,
    start_address: LPTHREAD_START_ROUTINE,
    #[builder(setter(strip_option))]
    parameter_address: Option<usize>,
    #[builder(setter(strip_option))]
    creation_flags: Option<u32>,
    #[builder(default, setter(skip))]
    thread_id: u32,
}

impl FnOnce<()> for CreateRemoteThread<'_> {
    type Output = Result<(u32, Handle)>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::CreateRemoteThread;

        #[allow(non_snake_case)]
        unsafe {
            let hProcess = self.process.as_raw_handle();
            let lpThreadAttributes = self
                .thread_attributes
                .map_or(null_mut(), |thread_attributes| thread_attributes as _);
            let dwStackSize = self.stack_size.unwrap_or_default();
            let lpStartAddress = self.start_address;
            let lpParameter = self
                .parameter_address
                .map_or(null_mut(), |parameter_address| parameter_address as _);
            let dwCreationFlags = self.creation_flags.unwrap_or_default();
            let lpThreadId = &mut self.thread_id;
            let r#return = CreateRemoteThread(
                hProcess,
                lpThreadAttributes,
                dwStackSize,
                lpStartAddress,
                lpParameter,
                dwCreationFlags,
                lpThreadId,
            );
            ensure!(!r#return.is_null(), io::Error::last_os_error());
            Ok((self.thread_id, Handle::from_raw_handle(r#return)))
        }
    }
}
