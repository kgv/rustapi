use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Write process memory.
#[derive(FnOnce, TypedBuilder)]
pub struct WriteProcessMemory<'a> {
    process: &'a Handle,
    base_address: usize,
    buffer: &'a [u8],
    #[builder(default, setter(skip))]
    number_of_bytes_written: usize,
}

impl FnOnce<()> for WriteProcessMemory<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::WriteProcessMemory;

        #[allow(non_snake_case)]
        unsafe {
            let hProcess = self.process.as_raw_handle();
            let lpBaseAddress = self.base_address as _;
            let lpBuffer = self.buffer.as_ptr() as _;
            let nSize = self.buffer.len();
            let lpNumberOfBytesWritten = &mut self.number_of_bytes_written;
            let r#return = WriteProcessMemory(
                hProcess,
                lpBaseAddress,
                lpBuffer,
                nSize,
                lpNumberOfBytesWritten,
            );
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.number_of_bytes_written)
        }
    }
}
