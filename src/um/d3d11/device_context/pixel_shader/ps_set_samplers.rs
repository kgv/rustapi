use crate::r#macro::FnOnce;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{
    ID3D11DeviceContext, ID3D11SamplerState, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT,
};
use wio::com::ComPtr;

/// Set samplers.
#[derive(FnOnce, TypedBuilder)]
pub struct PSSetSamplers<'a> {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default)]
    start_slot: u32,
    samplers: &'a [ComPtr<ID3D11SamplerState>],
}

impl FnOnce<()> for PSSetSamplers<'_> {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let StartSlot = self.start_slot;
            assert!(StartSlot <= D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1);
            let NumSamplers = self.samplers.len() as _;
            assert!(NumSamplers <= D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot);
            let ppSamplers = self.samplers.as_ptr() as _;
            self.device_context
                .PSSetSamplers(StartSlot, NumSamplers, ppSamplers);
        }
    }
}
