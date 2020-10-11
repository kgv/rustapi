use crate::r#macro::FnOnce;
use std::ptr::null_mut;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{ID3D11DeviceContext, ID3D11PixelShader, ID3D11ShaderResourceView};
use wio::com::ComPtr;

/// Set shader.
#[derive(FnOnce, TypedBuilder)]
pub struct PSSetShader<'a> {
    device_context: ComPtr<ID3D11DeviceContext>,
    pixel_shader: Option<ComPtr<ID3D11PixelShader>>,
    #[builder(default)]
    class_instances: &'a [ComPtr<ID3D11ShaderResourceView>],
}

impl FnOnce<()> for PSSetShader<'_> {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pPixelShader = self
                .pixel_shader
                .as_ref()
                .map_or(null_mut(), |v| v.as_raw());
            let ppClassInstances = self.class_instances.as_ptr() as _;
            let NumClassInstances = self.class_instances.len() as _;
            self.device_context
                .PSSetShader(pPixelShader, ppClassInstances, NumClassInstances);
        }
    }
}
