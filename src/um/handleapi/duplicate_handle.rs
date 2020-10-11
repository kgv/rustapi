use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Duplicate handle.
#[derive(FnOnce, TypedBuilder)]
pub struct DuplicateHandle<'a> {
    source_process_handle: &'a Handle,
    source_handle: &'a Handle,
    target_process_handle: &'a Handle,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    target_handle: MaybeUninit<Handle>,
    #[builder(default, setter(strip_option))]
    desired_access: Option<u32>,
    #[builder(default, setter(strip_option))]
    inherit_handle: Option<bool>,
    #[builder(default, setter(strip_option))]
    options: Option<u32>,
}

impl FnOnce<()> for DuplicateHandle<'_> {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::handleapi::DuplicateHandle;

        #[allow(non_snake_case)]
        unsafe {
            let hSourceProcessHandle = self.source_process_handle.as_raw_handle();
            let hSourceHandle = self.source_handle.as_raw_handle();
            let hTargetProcessHandle = self.target_process_handle.as_raw_handle();
            let lpTargetHandle = self.target_handle.as_mut_ptr() as _;
            let dwDesiredAccess = self.desired_access.unwrap_or_default();
            let bInheritHandle = self.inherit_handle.map_or(FALSE, |v| v as _);
            let dwOptions = self.options.unwrap_or_default();
            let r#return = DuplicateHandle(
                hSourceProcessHandle,
                hSourceHandle,
                hTargetProcessHandle,
                lpTargetHandle,
                dwDesiredAccess,
                bInheritHandle,
                dwOptions,
            );
            ensure!(r#return != FALSE, io::Error::last_os_error());
            let target_handle = self.target_handle.assume_init();
            Ok(target_handle)
        }
    }
}
