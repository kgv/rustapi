use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::{mem::MaybeUninit, ptr::null_mut};
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::SUCCEEDED,
    um::{
        d3d11::{ID3D11ClassLinkage, ID3D11Device, ID3D11PixelShader},
        d3dcommon::ID3DBlob,
    },
};
use wio::com::ComPtr;

/// Create pixel shader.
#[derive(FnOnce, TypedBuilder)]
pub struct CreatePixelShader {
    device: ComPtr<ID3D11Device>,
    bytecode: ComPtr<ID3DBlob>,
    #[builder(default)]
    linkage: Option<ComPtr<ID3D11ClassLinkage>>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    pixel_shader: MaybeUninit<ComPtr<ID3D11PixelShader>>,
}

impl FnOnce<()> for CreatePixelShader {
    type Output = Result<ComPtr<ID3D11PixelShader>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pShaderBytecode = self.bytecode.GetBufferPointer();
            let BytecodeLength = self.bytecode.GetBufferSize();
            let pClassLinkage = self.linkage.as_ref().map_or(null_mut(), |v| v.as_raw());
            let ppPixelShader = self.pixel_shader.as_mut_ptr() as _;
            let r#return = self.device.CreatePixelShader(
                pShaderBytecode,
                BytecodeLength,
                pClassLinkage,
                ppPixelShader,
            );
            ensure!(
                SUCCEEDED(r#return),
                "The ID3D11Device::CreatePixelShader call FAILED ({:#x}).",
                r#return,
            );
            Ok(self.pixel_shader.assume_init())
        }
    }
}
