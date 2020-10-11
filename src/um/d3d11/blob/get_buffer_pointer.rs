use crate::r#macro::FnOnce;
use std::ffi::c_void;
use typed_builder::TypedBuilder;
use winapi::um::d3dcommon::ID3DBlob;
use wio::com::ComPtr;

/// Get buffer pointer.
#[derive(FnOnce, TypedBuilder)]
pub struct GetBufferPointer {
    blob: ComPtr<ID3DBlob>,
}

impl FnOnce<()> for GetBufferPointer {
    type Output = *mut c_void;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let r#return = self.blob.GetBufferPointer();
            r#return
        }
    }
}
