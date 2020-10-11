use crate::{r#macro::FnOnce, shared::minwindef::ModuleHandle, utils::AsStrictRawHandle};
use anyhow::{ensure, Result};
use std::{ffi::CString, io};
use typed_builder::TypedBuilder;

/// Get process address.
#[derive(FnOnce, TypedBuilder)]
pub struct GetProcAddress<'a> {
    handle: &'a ModuleHandle,
    name: &'a str,
}

impl FnOnce<()> for GetProcAddress<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::libloaderapi::GetProcAddress;

        let name = CString::new(self.name)?;

        #[allow(non_snake_case)]
        unsafe {
            let hModule = self.handle.as_strict_raw_handle();
            let lpProcName = name.as_ptr() as _;
            let r#return = GetProcAddress(hModule, lpProcName);
            ensure!(!r#return.is_null(), io::Error::last_os_error());
            Ok(r#return as _)
        }
    }
}
