use crate::shared::ntdef::Handle;
use anyhow::{ensure, Result};
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Duplicate handle.
#[derive(TypedBuilder)]
pub struct DuplicateHandle<'a> {
    source_process_handle: &'a Handle,
    source_handle: &'a Handle,
    target_process_handle: &'a Handle,
    #[builder(default = MaybeUninit::uninit(), setter(skip))]
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
        let hSourceProcessHandle = self.source_process_handle.as_raw_handle();
        #[allow(non_snake_case)]
        let hSourceHandle = self.source_handle.as_raw_handle();
        #[allow(non_snake_case)]
        let hTargetProcessHandle = self.target_process_handle.as_raw_handle();
        #[allow(non_snake_case)]
        let lpTargetHandle = self.target_handle.as_mut_ptr() as _;
        #[allow(non_snake_case)]
        let dwDesiredAccess = self.desired_access.unwrap_or_default();
        #[allow(non_snake_case)]
        let bInheritHandle = self.inherit_handle.map_or(FALSE, |v| v as _);
        #[allow(non_snake_case)]
        let dwOptions = self.options.unwrap_or_default();
        let r#return = unsafe {
            DuplicateHandle(
                hSourceProcessHandle,
                hSourceHandle,
                hTargetProcessHandle,
                lpTargetHandle,
                dwDesiredAccess,
                bInheritHandle,
                dwOptions,
            )
        };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        let target_handle = unsafe { self.target_handle.assume_init() };
        Ok(target_handle)
    }
}
