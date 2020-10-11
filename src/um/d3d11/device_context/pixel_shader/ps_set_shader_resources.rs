use crate::r#macro::FnOnce;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{
    ID3D11DeviceContext, ID3D11ShaderResourceView, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT,
};
use wio::com::ComPtr;

/// Set shader resources.
#[derive(FnOnce, TypedBuilder)]
pub struct PSSetShaderResources<'a> {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default)]
    start_slot: u32,
    shader_resource_views: &'a [ComPtr<ID3D11ShaderResourceView>],
}

impl FnOnce<()> for PSSetShaderResources<'_> {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let StartSlot = self.start_slot;
            assert!(StartSlot <= D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1);
            let NumViews = self.shader_resource_views.len() as _;
            assert!(NumViews <= D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot);
            let ppShaderResourceViews = self.shader_resource_views.as_ptr() as _;
            self.device_context
                .PSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
        }
    }
}
