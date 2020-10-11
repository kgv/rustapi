use self::{get_buffer_pointer::GetBufferPointer, get_buffer_size::GetBufferSize};
use std::{ffi::c_void, slice::from_raw_parts};
use winapi::um::d3dcommon::ID3DBlob;
use wio::com::ComPtr;

/// Blob.
pub trait Blob {
    fn buffer(&self) -> &[u8];

    fn get_buffer_pointer(&self) -> *mut c_void;

    fn get_buffer_size(&self) -> usize;
}

impl Blob for ComPtr<ID3DBlob> {
    fn buffer(&self) -> &[u8] {
        let data = GetBufferPointer::builder().blob(self.clone()).build()();
        let len = GetBufferSize::builder().blob(self.clone()).build()();
        unsafe { from_raw_parts(data as _, len) }
    }

    fn get_buffer_pointer(&self) -> *mut c_void {
        GetBufferPointer::builder().blob(self.clone()).build()()
    }

    fn get_buffer_size(&self) -> usize {
        GetBufferSize::builder().blob(self.clone()).build()()
    }
}

mod get_buffer_pointer;
mod get_buffer_size;
