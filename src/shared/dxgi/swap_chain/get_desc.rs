use super::Description;
use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::shared::{dxgi::IDXGISwapChain, winerror::SUCCEEDED};
use wio::com::ComPtr;

/// Get description.
#[derive(FnOnce, TypedBuilder)]
pub struct GetDesc {
    swap_chain: ComPtr<IDXGISwapChain>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    description: MaybeUninit<Description>,
}

impl FnOnce<()> for GetDesc {
    type Output = Result<Description>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pDesc = self.description.as_mut_ptr() as _;
            let r#return = self.swap_chain.GetDesc(pDesc);
            ensure!(
                SUCCEEDED(r#return),
                "The IDXGISwapChain::GetDesc call FAILED ({}).",
                r#return,
            );
            Ok(self.description.assume_init())
        }
    }
}
