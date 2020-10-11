use crate::r#macro::FnOnce;
use std::ptr::null_mut;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{ID3D11DepthStencilState, ID3D11DeviceContext};
use wio::com::ComPtr;

/// Set depth stencil state.
#[derive(FnOnce, TypedBuilder)]
pub struct OMSetDepthStencilState {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default)]
    depth_stencil_state: Option<ComPtr<ID3D11DepthStencilState>>,
    stencil_reference: u32,
}

impl FnOnce<()> for OMSetDepthStencilState {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pDepthStencilState = self
                .depth_stencil_state
                .as_ref()
                .map_or(null_mut(), |v| v.as_raw());
            let StencilRef = self.stencil_reference;
            self.device_context
                .OMSetDepthStencilState(pDepthStencilState, StencilRef);
        }
    }
}
