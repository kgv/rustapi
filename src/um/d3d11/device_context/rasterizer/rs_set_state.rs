use crate::r#macro::FnOnce;
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{ID3D11DeviceContext, ID3D11RasterizerState};
use wio::com::ComPtr;

/// Set state.
#[derive(FnOnce, TypedBuilder)]
pub struct RSSetState {
    device_context: ComPtr<ID3D11DeviceContext>,
    rasterizer_state: ComPtr<ID3D11RasterizerState>,
}

impl FnOnce<()> for RSSetState {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let pRasterizerState = self.rasterizer_state.as_raw();
            self.device_context.RSSetState(pRasterizerState);
        }
    }
}
