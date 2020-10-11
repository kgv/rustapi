use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11ClassLinkage, ID3D11Device},
};
use wio::com::ComPtr;

/// Create class linkage.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateClassLinkage {
    device: ComPtr<ID3D11Device>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    linkage: MaybeUninit<ComPtr<ID3D11ClassLinkage>>,
}

impl FnOnce<()> for CreateClassLinkage {
    type Output = Result<ComPtr<ID3D11ClassLinkage>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let ppLinkage = self.linkage.as_mut_ptr() as _;
            let r#return = self.device.CreateClassLinkage(ppLinkage);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateClassLinkage call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.linkage.assume_init())
        }
    }
}
