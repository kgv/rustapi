use crate::{r#macro::FnOnce, um::d3d11::depth_stencil_state::Description};
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11DepthStencilState, ID3D11Device},
};
use wio::com::ComPtr;

/// Create depth stencil state.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateDepthStencilState {
    device: ComPtr<ID3D11Device>,
    #[builder(setter(into))]
    description: Description,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    depth_stencil_state: MaybeUninit<ComPtr<ID3D11DepthStencilState>>,
}

impl FnOnce<()> for CreateDepthStencilState {
    type Output = Result<ComPtr<ID3D11DepthStencilState>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pDepthStencilDesc = &*self.description;
            let ppDepthStencilState = self.depth_stencil_state.as_mut_ptr() as _;
            let r#return = self
                .device
                .CreateDepthStencilState(pDepthStencilDesc, ppDepthStencilState);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateDepthStencilState call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.depth_stencil_state.assume_init())
        }
    }
}
