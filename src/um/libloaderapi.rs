use crate::{
    shared::minwindef::ModuleHandle,
    utils::{AsStrictRawHandle, FromStrictRawHandle},
};
use anyhow::{ensure, Result};
use std::{ffi::CString, io};
use typed_builder::TypedBuilder;
use widestring::WideCString;
use winapi::shared::minwindef::FALSE;

/// Disable thread library calls.
#[derive(TypedBuilder)]
pub struct DisableThreadLibraryCalls<'a> {
    handle: &'a ModuleHandle,
}

impl FnOnce<()> for DisableThreadLibraryCalls<'_> {
    type Output = Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::libloaderapi::DisableThreadLibraryCalls;

        #[allow(non_snake_case)]
        let hLibModule = self.handle.as_strict_raw_handle();
        let r#return = unsafe { DisableThreadLibraryCalls(hLibModule) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(())
    }
}

/// Get module handle.
#[derive(TypedBuilder)]
pub struct GetModuleHandle<'a> {
    module_name: &'a str,
}

impl FnOnce<()> for GetModuleHandle<'_> {
    type Output = Result<ModuleHandle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::libloaderapi::GetModuleHandleW;

        let module_name = WideCString::from_str(self.module_name)?;

        #[allow(non_snake_case)]
        let lpModuleName = module_name.as_ptr();
        let r#return = unsafe { GetModuleHandleW(lpModuleName) };
        ensure!(!r#return.is_null(), io::Error::last_os_error());
        Ok(unsafe { ModuleHandle::from_strict_raw_handle(r#return) })
    }
}

/// Get process address.
#[derive(TypedBuilder)]
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
        let hModule = self.handle.as_strict_raw_handle();
        #[allow(non_snake_case)]
        let lpProcName = name.as_ptr() as _;
        let r#return = unsafe { GetProcAddress(hModule, lpProcName) };
        ensure!(!r#return.is_null(), io::Error::last_os_error());
        Ok(r#return as _)
    }
}
