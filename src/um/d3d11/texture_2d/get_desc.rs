use super::Description;
use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::ID3D11Texture2D;
use wio::com::ComPtr;

/// Get texture 2D description.
#[derive(FnOnce, TypedBuilder)]
pub struct GetDesc {
    texture_2d: ComPtr<ID3D11Texture2D>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    description: MaybeUninit<Description>,
}

impl FnOnce<()> for GetDesc {
    type Output = Description;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pDesc = self.description.as_mut_ptr() as _;
            self.texture_2d.GetDesc(pDesc);
            self.description.assume_init()
        }
    }
}
