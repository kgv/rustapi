use crate::{r#macro::FnOnce, um::d3d11::shader_resource_view::Description};
use anyhow::{ensure, Result};
use std::{mem::MaybeUninit, ptr::null};
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::d3d11::{ID3D11Device, ID3D11Resource, ID3D11ShaderResourceView},
};
use wio::com::ComPtr;

/// Create shader resource view.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateShaderResourceView {
    device: ComPtr<ID3D11Device>,
    resource: ComPtr<ID3D11Resource>,
    #[builder(setter(into, strip_option))]
    description: Option<Description>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    shader_resource_view: MaybeUninit<ComPtr<ID3D11ShaderResourceView>>,
}

impl FnOnce<()> for CreateShaderResourceView {
    type Output = Result<ComPtr<ID3D11ShaderResourceView>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pResource = self.resource.as_raw();
            let pDesc = self.description.as_deref().map_or(null(), |v| v);
            let ppSRView = self.shader_resource_view.as_mut_ptr() as _;
            let r#return = self
                .device
                .CreateShaderResourceView(pResource, pDesc, ppSRView);
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreateShaderResourceView call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.shader_resource_view.assume_init())
        }
    }
}
