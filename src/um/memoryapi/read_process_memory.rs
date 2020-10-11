use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Read process memory.
#[derive(FnOnce, TypedBuilder)]
pub struct ReadProcessMemory<'a> {
    process: &'a Handle,
    base_address: &'a usize,
    buffer: &'a mut [u8],
    #[builder(default, setter(skip))]
    number_of_bytes_read: usize,
}

impl FnOnce<()> for ReadProcessMemory<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::ReadProcessMemory;

        #[allow(non_snake_case)]
        unsafe {
            let hProcess = self.process.as_raw_handle();
            let lpBaseAddress = self.base_address as *const _ as _;
            let lpBuffer = self.buffer.as_mut_ptr() as _;
            let nSize = self.buffer.len();
            let lpNumberOfBytesRead = &mut self.number_of_bytes_read;
            let r#return = ReadProcessMemory(
                hProcess,
                lpBaseAddress,
                lpBuffer,
                nSize,
                lpNumberOfBytesRead,
            );
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.number_of_bytes_read)
        }
    }
}
