use anyhow::{ensure, Result};
use std::{
    default::Default, fs::File, io, mem::size_of_val, os::windows::io::AsRawHandle, ptr::null_mut,
};
use typed_builder::TypedBuilder;
use winapi::{shared::minwindef::FALSE, um::minwinbase::OVERLAPPED};

/// Device io control.
#[derive(TypedBuilder)]
pub struct DeviceIoControl<'a> {
    device: &'a File,
    #[builder(default)]
    io_control_code: u32,
    #[builder(default, setter(strip_option))]
    in_buffer: Option<&'a mut [u8]>,
    #[builder(default, setter(strip_option))]
    out_buffer: Option<&'a mut [u8]>,
    #[builder(default, setter(skip))]
    bytes_returned: u32,
    #[builder(default, setter(strip_option))]
    overlapped: Option<&'a mut OVERLAPPED>,
}

impl FnOnce<()> for DeviceIoControl<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::ioapiset::DeviceIoControl;

        #[allow(non_snake_case)]
        let hDevice = self.device.as_raw_handle();
        #[allow(non_snake_case)]
        let dwIoControlCode = self.io_control_code;
        #[allow(non_snake_case)]
        let nInBufferSize = self.in_buffer.as_ref().map_or(0, |v| size_of_val(*v) as _);
        #[allow(non_snake_case)]
        let lpInBuffer = self.in_buffer.map_or(null_mut(), |v| v.as_mut_ptr() as _);
        #[allow(non_snake_case)]
        let nOutBufferSize = self.out_buffer.as_ref().map_or(0, |v| size_of_val(*v) as _);
        #[allow(non_snake_case)]
        let lpOutBuffer = self.out_buffer.map_or(null_mut(), |v| v.as_mut_ptr() as _);
        #[allow(non_snake_case)]
        let lpBytesReturned = &mut self.bytes_returned;
        #[allow(non_snake_case)]
        let lpOverlapped = self.overlapped.map_or(null_mut(), |v| v as _);
        let r#return = unsafe {
            DeviceIoControl(
                hDevice,
                dwIoControlCode,
                lpInBuffer,
                nInBufferSize,
                lpOutBuffer,
                nOutBufferSize,
                lpBytesReturned,
                lpOverlapped,
            )
        };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.bytes_returned as _)
    }
}
