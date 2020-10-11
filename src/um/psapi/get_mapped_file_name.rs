use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle, path::PathBuf};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::MAX_PATH;

/// Get mapped file name.
#[derive(FnOnce, TypedBuilder)]
pub struct GetMappedFileName<'a> {
    process: &'a Handle,
    address: usize,
    #[builder(default = Vec::with_capacity(MAX_PATH), setter(skip))]
    file_name: Vec<u16>,
}

impl FnOnce<()> for GetMappedFileName<'_> {
    type Output = Result<PathBuf>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::psapi::GetMappedFileNameW;

        #[allow(non_snake_case)]
        unsafe {
            let hProcess = self.process.as_raw_handle();
            let lpv = self.address as _;
            let lpFilename = self.file_name.as_mut_ptr() as _;
            let nSize = self.file_name.capacity() as _;
            let r#return = GetMappedFileNameW(hProcess, lpv, lpFilename, nSize);
            ensure!(r#return != 0, io::Error::last_os_error());
            self.file_name.set_len(r#return as _);
            Ok(PathBuf::from(String::from_utf16(&self.file_name)?))
        }
    }
}
