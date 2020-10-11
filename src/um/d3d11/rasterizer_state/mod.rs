use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::{ID3D11RasterizerState, D3D11_RASTERIZER_DESC};
use wio::com::ComPtr;

/// Rasterizer state.
pub trait RasterizerState {
    fn get_description(&self) -> Description;
}

impl RasterizerState for ComPtr<ID3D11RasterizerState> {
    fn get_description(&self) -> Description {
        GetDesc::builder().rasterizer_state(self.clone()).build()()
    }
}

/// Rasterizer state description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_RASTERIZER_DESC);

mod get_desc;
