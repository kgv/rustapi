use crate::r#macro::FnOnce;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{
    ID3D11Buffer, ID3D11DeviceContext, D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT,
};
use wio::com::ComPtr;

/// Constant buffers count.
const CAPACITY: usize = D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT as _;

/// Get constant buffers.
#[derive(FnOnce, TypedBuilder)]
pub struct VSGetConstantBuffers {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default)]
    start_slot: u32,
    #[builder(default = D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT)]
    number_of_buffers: u32,
    #[builder(default = Vec::with_capacity(CAPACITY), setter(skip))]
    constant_buffers: Vec<ComPtr<ID3D11Buffer>>,
}

impl FnOnce<()> for VSGetConstantBuffers {
    type Output = Vec<ComPtr<ID3D11Buffer>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let StartSlot = self.start_slot;
            assert!(StartSlot <= D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1);
            let NumBuffers = self.number_of_buffers;
            assert!(NumBuffers <= D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot);
            let ppConstantBuffers = self.constant_buffers.as_mut_ptr() as _;
            self.device_context
                .VSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
            self.constant_buffers.set_len(self.number_of_buffers as _);
            self.constant_buffers
        }
    }
}
