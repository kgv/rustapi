use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::{dxgi::IDXGISwapChain, winerror::SUCCEEDED},
    Interface,
};
use wio::com::ComPtr;

/// Get buffer.
#[derive(FnOnce, TypedBuilder)]
pub struct GetBuffer<T: Interface> {
    swap_chain: ComPtr<IDXGISwapChain>,
    index: u32,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    surface: MaybeUninit<ComPtr<T>>,
}

impl<T: Interface> FnOnce<()> for GetBuffer<T> {
    type Output = Result<ComPtr<T>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let Buffer = self.index;
            let riid = &T::uuidof();
            let ppSurface = self.surface.as_mut_ptr() as _;
            let r#return = self.swap_chain.GetBuffer(Buffer, riid, ppSurface);
            ensure!(
                SUCCEEDED(r#return),
                "The IDXGISwapChain::GetBuffer call FAILED ({}).",
                r#return
            );
            Ok(self.surface.assume_init())
        }
    }
}
