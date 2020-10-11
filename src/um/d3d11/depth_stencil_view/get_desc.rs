use super::Description;
use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::ID3D11DepthStencilView;
use wio::com::ComPtr;

/// Get depth stencil view description.
#[derive(FnOnce, TypedBuilder)]
pub struct GetDesc {
    depth_stencil_view: ComPtr<ID3D11DepthStencilView>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    description: MaybeUninit<Description>,
}

impl FnOnce<()> for GetDesc {
    type Output = Description;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pDesc = self.description.as_mut_ptr() as _;
            self.depth_stencil_view.GetDesc(pDesc);
            self.description.assume_init()
        }
    }
}
