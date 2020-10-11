use crate::{r#macro::FnOnce, um::d3d11::rasterizer_state::Description};
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11Device, ID3D11RasterizerState},
};
use wio::com::ComPtr;

/// Create rasterizer state.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateRasterizerState {
    device: ComPtr<ID3D11Device>,
    #[builder(setter(into))]
    description: Description,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    rasterizer_state: MaybeUninit<ComPtr<ID3D11RasterizerState>>,
}

impl FnOnce<()> for CreateRasterizerState {
    type Output = Result<ComPtr<ID3D11RasterizerState>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pRasterizerDesc = &*self.description;
            let ppRasterizerState = self.rasterizer_state.as_mut_ptr() as _;
            let r#return = self
                .device
                .CreateRasterizerState(pRasterizerDesc, ppRasterizerState);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateRasterizerState call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.rasterizer_state.assume_init())
        }
    }
}
