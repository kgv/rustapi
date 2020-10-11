use crate::{r#macro::FnOnce, um::d3d11::render_target_view::Description};
use anyhow::{ensure, Result};
use std::{mem::MaybeUninit, ptr::null};
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11Device, ID3D11RenderTargetView, ID3D11Resource},
};
use wio::com::ComPtr;

/// Create render target view.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateRenderTargetView {
    device: ComPtr<ID3D11Device>,
    resource: ComPtr<ID3D11Resource>,
    #[builder(setter(into, strip_option))]
    description: Option<Description>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    render_target_view: MaybeUninit<ComPtr<ID3D11RenderTargetView>>,
}

impl FnOnce<()> for CreateRenderTargetView {
    type Output = Result<ComPtr<ID3D11RenderTargetView>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pResource = self.resource.as_raw();
            let pDesc = self.description.as_deref().map_or(null(), |v| v);
            let ppRTView = self.render_target_view.as_mut_ptr() as _;
            let r#return = self
                .device
                .CreateRenderTargetView(pResource, pDesc, ppRTView);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateRenderTargetView call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.render_target_view.assume_init())
        }
    }
}
