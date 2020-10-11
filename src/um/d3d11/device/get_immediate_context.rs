use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{ID3D11Device, ID3D11DeviceContext};
use wio::com::ComPtr;

/// Get immediate context.
#[derive(FnOnce, TypedBuilder)]
pub struct GetImmediateContext {
    device: ComPtr<ID3D11Device>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    context: MaybeUninit<ComPtr<ID3D11DeviceContext>>,
}

impl FnOnce<()> for GetImmediateContext {
    type Output = ComPtr<ID3D11DeviceContext>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let ppImmediateContext = self.context.as_mut_ptr() as _;
            self.device.GetImmediateContext(ppImmediateContext);
            self.context.assume_init()
        }
    }
}
