use crate::r#macro::FnOnce;
use typed_builder::TypedBuilder;
use winapi::um::d3dcommon::ID3DBlob;
use wio::com::ComPtr;

/// Get buffer size.
#[derive(FnOnce, TypedBuilder)]
pub struct GetBufferSize {
    blob: ComPtr<ID3DBlob>,
}

impl FnOnce<()> for GetBufferSize {
    type Output = usize;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let r#return = self.blob.GetBufferSize();
            r#return
        }
    }
}
