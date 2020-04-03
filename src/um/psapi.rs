use crate::shared::ntdef::Handle;
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle, path::PathBuf};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::MAX_PATH;

/// Get mapped file name.
#[derive(TypedBuilder)]
pub struct GetMappedFileName<'a> {
    process: &'a Handle,
    address: usize,
    #[builder(default = Vec::with_capacity(MAX_PATH), setter(skip))]
    buffer: Vec<u16>,
}

impl FnOnce<()> for GetMappedFileName<'_> {
    type Output = Result<PathBuf>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::psapi::GetMappedFileNameW;

        #[allow(non_snake_case)]
        let hProcess = self.process.as_raw_handle();
        #[allow(non_snake_case)]
        let lpv = self.address as _;
        #[allow(non_snake_case)]
        let lpFilename = self.buffer.as_mut_ptr() as _;
        #[allow(non_snake_case)]
        let nSize = self.buffer.capacity() as _;
        let r#return = unsafe { GetMappedFileNameW(hProcess, lpv, lpFilename, nSize) };
        ensure!(r#return != 0, io::Error::last_os_error());
        unsafe { self.buffer.set_len(r#return as _) };
        Ok(PathBuf::from(String::from_utf16(&self.buffer)?))
    }
}
