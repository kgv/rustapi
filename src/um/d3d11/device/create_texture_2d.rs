use crate::{r#macro::FnOnce, um::d3d11::texture_2d::Description, utils::Transparent};
use anyhow::{ensure, Result};
use std::{mem::MaybeUninit, ptr::null};
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11Device, ID3D11Texture2D, D3D11_SUBRESOURCE_DATA},
};
use wio::com::ComPtr;

/// Create texture 2D.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateTexture2D<'a, T>
where
    T: Transparent<Target = D3D11_SUBRESOURCE_DATA>,
{
    device: ComPtr<ID3D11Device>,
    #[builder(setter(into))]
    description: Description,
    #[builder(setter(strip_option))]
    initial_data: Option<&'a [T]>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    texture_2d: MaybeUninit<ComPtr<ID3D11Texture2D>>,
}

impl<T> FnOnce<()> for CreateTexture2D<'_, T>
where
    T: Transparent<Target = D3D11_SUBRESOURCE_DATA>,
{
    type Output = Result<ComPtr<ID3D11Texture2D>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pDesc = &*self.description;
            let pInitialData = self.initial_data.map_or(null(), |v| v.as_ptr() as _);
            let ppTexture2D = self.texture_2d.as_mut_ptr() as _;
            let r#return = self
                .device
                .CreateTexture2D(pDesc, pInitialData, ppTexture2D);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateTexture2D call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.texture_2d.assume_init())
        }
    }
}
