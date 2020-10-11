use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use itertools::Itertools;
use log::warn;
use std::{io, ptr::null};
use typed_builder::TypedBuilder;
use widestring::{WideCStr, WideCString};
use winapi::{
    self,
    shared::{minwindef::MAX_PATH, winerror::ERROR_INSUFFICIENT_BUFFER},
};

/// Query dos device.
#[derive(FnOnce, TypedBuilder)]
pub struct QueryDosDevice<'a> {
    #[builder(default, setter(strip_option))]
    device_name: Option<&'a str>,
    #[builder(default = Vec::with_capacity(MAX_PATH))]
    target_path: Vec<u16>,
}

impl Default for QueryDosDevice<'_> {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl FnOnce<()> for QueryDosDevice<'_> {
    type Output = Result<Vec<String>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::fileapi::QueryDosDeviceW;

        let device_name = self.device_name.map(WideCString::from_str).transpose()?;

        loop {
            #[allow(non_snake_case)]
            unsafe {
                let lpDeviceName = device_name.as_ref().map_or(null(), |v| v.as_ptr());
                let lpTargetPath = self.target_path.as_mut_ptr() as _;
                let ucchMax = self.target_path.capacity() as _;
                let r#return = QueryDosDeviceW(lpDeviceName, lpTargetPath, ucchMax);
                let last_os_error = io::Error::last_os_error();
                if r#return != 0 {
                    let length = r#return as usize;
                    self.target_path.set_len(length - 1);
                    break;
                }
                let raw_os_error = last_os_error.raw_os_error().unwrap() as _;
                ensure!(ERROR_INSUFFICIENT_BUFFER == raw_os_error, last_os_error);
                warn!("{}", last_os_error);
                let additional = self.target_path.capacity() * 2 - self.target_path.len();
                self.target_path.reserve(additional);
            }
        }
        let strings = self
            .target_path
            .split_inclusive(|&v| v == 0)
            .map(WideCStr::from_slice_with_nul)
            .map_results(|v| v.to_string())
            .flatten()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(strings)
    }
}
