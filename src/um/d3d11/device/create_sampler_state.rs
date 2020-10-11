use crate::{r#macro::FnOnce, um::d3d11::sampler_state::Description};
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11Device, ID3D11SamplerState},
};
use wio::com::ComPtr;

/// Create sampler state.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateSamplerState {
    device: ComPtr<ID3D11Device>,
    #[builder(setter(into))]
    description: Description,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    sampler_state: MaybeUninit<ComPtr<ID3D11SamplerState>>,
}

impl FnOnce<()> for CreateSamplerState {
    type Output = Result<ComPtr<ID3D11SamplerState>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pSamplerDesc = &*self.description;
            let ppSamplerState = self.sampler_state.as_mut_ptr() as _;
            let r#return = self.device.CreateSamplerState(pSamplerDesc, ppSamplerState);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateSamplerState call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.sampler_state.assume_init())
        }
    }
}
