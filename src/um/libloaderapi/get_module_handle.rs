use crate::{r#macro::FnOnce, shared::minwindef::ModuleHandle, utils::FromStrictRawHandle};
use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;
use widestring::WideCString;

/// Get module handle.
#[derive(FnOnce, TypedBuilder)]
pub struct GetModuleHandle<'a> {
    name: &'a str,
}

impl FnOnce<()> for GetModuleHandle<'_> {
    type Output = Result<ModuleHandle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::libloaderapi::GetModuleHandleW;

        let name = WideCString::from_str(self.name)?;

        #[allow(non_snake_case)]
        unsafe {
            let lpModuleName = name.as_ptr();
            let r#return = GetModuleHandleW(lpModuleName);
            ensure!(!r#return.is_null(), io::Error::last_os_error());
            Ok(ModuleHandle::from_strict_raw_handle(r#return))
        }
    }
}
