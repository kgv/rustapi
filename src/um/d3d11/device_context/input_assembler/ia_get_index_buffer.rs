use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::dxgiformat::DXGI_FORMAT,
    um::d3d11::{ID3D11Buffer, ID3D11DeviceContext},
};
use wio::com::ComPtr;

/// Get index buffer.
#[derive(FnOnce, TypedBuilder)]
pub struct IAGetIndexBuffer {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    index_buffer: MaybeUninit<ComPtr<ID3D11Buffer>>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    format: MaybeUninit<DXGI_FORMAT>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    offset: MaybeUninit<u32>,
}

impl FnOnce<()> for IAGetIndexBuffer {
    type Output = (ComPtr<ID3D11Buffer>, DXGI_FORMAT, u32);

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pIndexBuffer = self.index_buffer.as_mut_ptr() as _;
            let Format = self.format.as_mut_ptr();
            let Offset = self.offset.as_mut_ptr();
            self.device_context
                .IAGetIndexBuffer(pIndexBuffer, Format, Offset);
            (
                self.index_buffer.assume_init(),
                self.format.assume_init(),
                self.offset.assume_init(),
            )
        }
    }
}
