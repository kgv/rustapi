use crate::r#macro::FnOnce;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{
    ID3D11Buffer, ID3D11DeviceContext, D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT,
};
use wio::com::ComPtr;

/// Vertex buffers count.
const CAPACITY: usize = D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT as _;

/// Get vertex buffers.
#[derive(FnOnce, TypedBuilder)]
pub struct IAGetVertexBuffers {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default)]
    start_slot: u32,
    #[builder(default = D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT)]
    number_of_buffers: u32,
    #[builder(default = Vec::with_capacity(CAPACITY), setter(skip))]
    vertex_buffers: Vec<ComPtr<ID3D11Buffer>>,
    #[builder(default = Vec::with_capacity(CAPACITY), setter(skip))]
    strides: Vec<u32>,
    #[builder(default = Vec::with_capacity(CAPACITY), setter(skip))]
    offsets: Vec<u32>,
}

impl FnOnce<()> for IAGetVertexBuffers {
    type Output = (Vec<ComPtr<ID3D11Buffer>>, Vec<u32>, Vec<u32>);

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let StartSlot = self.start_slot;
            assert!(StartSlot <= D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1);
            let NumBuffers = self.number_of_buffers;
            assert!(NumBuffers <= D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot);
            let ppVertexBuffers = self.vertex_buffers.as_mut_ptr() as _;
            let pStrides = self.strides.as_mut_ptr();
            let pOffsets = self.offsets.as_mut_ptr();
            self.device_context.IAGetVertexBuffers(
                StartSlot,
                NumBuffers,
                ppVertexBuffers,
                pStrides,
                pOffsets,
            );
            self.vertex_buffers.set_len(self.number_of_buffers as _);
            self.strides.set_len(self.number_of_buffers as _);
            self.offsets.set_len(self.number_of_buffers as _);
            (self.vertex_buffers, self.strides, self.offsets)
        }
    }
}
