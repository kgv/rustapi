use anyhow::{ensure, Result};
use itertools::Itertools;
use log::warn;
use std::{
    fs::File,
    io,
    os::windows::io::{AsRawHandle, FromRawHandle},
    ptr::{null, null_mut},
};
use typed_builder::TypedBuilder;
use widestring::{WideCStr, WideCString};
use winapi::{
    shared::{minwindef::MAX_PATH, winerror::ERROR_INSUFFICIENT_BUFFER},
    um::{
        handleapi::INVALID_HANDLE_VALUE, minwinbase::SECURITY_ATTRIBUTES,
        winnt::FILE_ATTRIBUTE_NORMAL,
    },
};

/// Create file.
#[derive(TypedBuilder)]
pub struct CreateFile<'a> {
    file_name: &'a str,
    access_mode: u32,
    #[builder(default)]
    share_mode: u32,
    #[builder(default, setter(strip_option))]
    security_attributes: Option<&'a mut SECURITY_ATTRIBUTES>,
    creation_disposition: u32,
    #[builder(default = FILE_ATTRIBUTE_NORMAL)]
    flags_and_attributes: u32,
    #[builder(default, setter(strip_option))]
    template_file: Option<&'a mut File>,
}

impl FnOnce<()> for CreateFile<'_> {
    type Output = Result<File>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::fileapi::CreateFileW;

        let file_name = WideCString::from_str(self.file_name)?;

        #[allow(non_snake_case)]
        let lpFileName = file_name.as_ptr();
        #[allow(non_snake_case)]
        let dwDesiredAccess = self.access_mode;
        #[allow(non_snake_case)]
        let dwShareMode = self.share_mode;
        #[allow(non_snake_case)]
        let lpSecurityAttributes = self.security_attributes.map_or(null_mut(), |v| v as _);
        #[allow(non_snake_case)]
        let dwCreationDisposition = self.creation_disposition;
        #[allow(non_snake_case)]
        let dwFlagsAndAttributes = self.flags_and_attributes;
        #[allow(non_snake_case)]
        let hTemplateFile = self.template_file.map_or(null_mut(), |v| v.as_raw_handle());
        let r#return = unsafe {
            CreateFileW(
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
        };
        ensure!(r#return != INVALID_HANDLE_VALUE, io::Error::last_os_error());
        Ok(unsafe { File::from_raw_handle(r#return) })
    }
}

/// Get logical drive strings.
#[derive(TypedBuilder)]
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
            let nBufferLength = self.buffer.capacity() as _;
            #[allow(non_snake_case)]
            let lpBuffer = self.buffer.as_mut_ptr() as _;
            let r#return = unsafe { GetLogicalDriveStringsW(nBufferLength, lpBuffer) };
            ensure!(r#return != 0, io::Error::last_os_error());
            let length = r#return as _;
            if length < self.buffer.capacity() {
                unsafe { self.buffer.set_len(length) };
                break;
            }
            let additional = length - self.buffer.len();
            self.buffer.reserve(additional);
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

/// Query dos device.
#[derive(TypedBuilder)]
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
            let lpDeviceName = device_name.as_ref().map_or(null(), |v| v.as_ptr());
            #[allow(non_snake_case)]
            let lpTargetPath = self.target_path.as_mut_ptr() as _;
            #[allow(non_snake_case)]
            let ucchMax = self.target_path.capacity() as _;
            let r#return = unsafe { QueryDosDeviceW(lpDeviceName, lpTargetPath, ucchMax) };
            let last_os_error = io::Error::last_os_error();
            if r#return != 0 {
                let length = r#return as usize;
                unsafe { self.target_path.set_len(length - 1) };
                break;
            }
            let raw_os_error = last_os_error.raw_os_error().unwrap() as _;
            ensure!(ERROR_INSUFFICIENT_BUFFER == raw_os_error, last_os_error);
            warn!("{}", last_os_error);
            let additional = self.target_path.capacity() * 2 - self.target_path.len();
            self.target_path.reserve(additional);
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

#[cfg(test)]
mod tests {
    use super::*;

    mod create_file {
        use super::CreateFile;
        use anyhow::Result;
        use winapi::um::{fileapi::CREATE_NEW, winnt::FILE_ALL_ACCESS};

        #[test]
        fn create_file() -> Result<()> {
            let _privilege = CreateFile::builder()
                .file_name(".rs.bk/test_w_file")
                .access_mode(FILE_ALL_ACCESS)
                .creation_disposition(CREATE_NEW)
                .build()()?;
            Ok(())
        }
    }

    mod get_logical_drive_strings {
        use super::GetLogicalDriveStrings;
        use anyhow::Result;

        #[test]
        fn default() -> Result<()> {
            let drives = GetLogicalDriveStrings::default()()?;
            assert!(drives.contains(&format!("C:\\")));
            println!("drives: {:?}", drives);
            Ok(())
        }

        #[test]
        fn with_buffer() -> Result<()> {
            let drives = GetLogicalDriveStrings::builder()
                .buffer(Vec::with_capacity(260))
                .build()()?;
            assert!(drives.contains(&format!("C:\\")));
            println!("drives: {:?}", drives);
            Ok(())
        }
    }

    mod query_dos_device {
        use super::QueryDosDevice;
        use anyhow::Result;

        #[test]
        fn default() -> Result<()> {
            let device = QueryDosDevice::default()()?;
            println!("device: {:#?}", device);
            Ok(())
        }

        #[test]
        fn with_device_name() -> Result<()> {
            let device = QueryDosDevice::builder().device_name("C:").build()()?;
            println!("device: {:#?}", device);
            Ok(())
        }

        #[test]
        fn with_device_name_and_target_path() -> Result<()> {
            let _device = QueryDosDevice::builder()
                .device_name("C:")
                .target_path(Vec::with_capacity(2))
                .build()()?;
            Ok(())
        }
    }
}
