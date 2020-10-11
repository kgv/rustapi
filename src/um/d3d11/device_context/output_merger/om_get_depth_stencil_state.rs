use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{ID3D11DepthStencilState, ID3D11DeviceContext};
use wio::com::ComPtr;

/// Get depth stencil state.
#[derive(FnOnce, TypedBuilder)]
pub struct OMGetDepthStencilState {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    depth_stencil_state: MaybeUninit<ComPtr<ID3D11DepthStencilState>>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    stencil_reference: MaybeUninit<u32>,
}

impl FnOnce<()> for OMGetDepthStencilState {
    type Output = (ComPtr<ID3D11DepthStencilState>, u32);

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let ppDepthStencilState = self.depth_stencil_state.as_mut_ptr() as _;
            let pStencilRef = self.stencil_reference.as_mut_ptr();
            self.device_context
                .OMGetDepthStencilState(ppDepthStencilState, pStencilRef);
            (
                self.depth_stencil_state.assume_init(),
                self.stencil_reference.assume_init(),
            )
        }
    }
}
