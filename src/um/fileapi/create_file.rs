use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::{
    fs::File,
    io,
    os::windows::io::{AsRawHandle, FromRawHandle},
    path::Path,
    ptr::null_mut,
};
use typed_builder::TypedBuilder;
use widestring::WideCString;
use winapi::um::{
    handleapi::INVALID_HANDLE_VALUE, minwinbase::SECURITY_ATTRIBUTES, winnt::FILE_ATTRIBUTE_NORMAL,
};

/// Create file.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateFile<'a> {
    file_name: &'a Path,
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

        let file_name = WideCString::from_os_str(self.file_name)?;

        #[allow(non_snake_case)]
        unsafe {
            let lpFileName = file_name.as_ptr();
            let dwDesiredAccess = self.access_mode;
            let dwShareMode = self.share_mode;
            let lpSecurityAttributes = self.security_attributes.map_or(null_mut(), |v| v as _);
            let dwCreationDisposition = self.creation_disposition;
            let dwFlagsAndAttributes = self.flags_and_attributes;
            let hTemplateFile = self.template_file.map_or(null_mut(), |v| v.as_raw_handle());
            let r#return = CreateFileW(
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            );
            ensure!(r#return != INVALID_HANDLE_VALUE, io::Error::last_os_error());
            Ok(File::from_raw_handle(r#return))
        }
    }
}
