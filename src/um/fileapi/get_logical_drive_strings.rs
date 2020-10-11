use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use itertools::Itertools;
use std::io;
use typed_builder::TypedBuilder;
use widestring::WideCStr;

/// Get logical drive strings.
#[derive(FnOnce, TypedBuilder)]
pub struct GetLogicalDriveStrings {
    #[builder(default)]
    buffer: Vec<u16>,
}

impl Default for GetLogicalDriveStrings {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl FnOnce<()> for GetLogicalDriveStrings {
    type Output = Result<Vec<String>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::fileapi::GetLogicalDriveStringsW;

        loop {
            #[allow(non_snake_case)]
            unsafe {
                let nBufferLength = self.buffer.capacity() as _;
                let lpBuffer = self.buffer.as_mut_ptr() as _;
                let r#return = GetLogicalDriveStringsW(nBufferLength, lpBuffer);
                ensure!(r#return != 0, io::Error::last_os_error());
                let length = r#return as _;
                if length < self.buffer.capacity() {
                    self.buffer.set_len(length);
                    break;
                }
                let additional = length - self.buffer.len();
                self.buffer.reserve(additional);
            }
        }
        let strings = self
            .buffer
            .split_inclusive(|&v| v == 0)
            .map(WideCStr::from_slice_with_nul)
            .map_results(|v| v.to_string())
            .flatten()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(strings)
    }
}
