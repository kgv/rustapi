use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::Result;
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Close handle.
#[derive(FnOnce, TypedBuilder)]
pub struct CloseHandle<'a> {
    handle: &'a Handle,
}

impl FnOnce<()> for CloseHandle<'_> {
    type Output = Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::handleapi::CloseHandle;

        #[allow(non_snake_case)]
        unsafe {
            let hObject = self.handle.as_raw_handle();
            let r#return = CloseHandle(hObject);
            assert!(r#return != FALSE, io::Error::last_os_error());
            Ok(())
        }
    }
}
