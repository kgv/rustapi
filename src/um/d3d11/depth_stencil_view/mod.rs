use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::{ID3D11DepthStencilView, D3D11_DEPTH_STENCIL_VIEW_DESC};
use wio::com::ComPtr;

/// Depth stencil view.
pub trait DepthStencilView {
    fn get_description(&self) -> Description;
}

impl DepthStencilView for ComPtr<ID3D11DepthStencilView> {
    fn get_description(&self) -> Description {
        GetDesc::builder().depth_stencil_view(self.clone()).build()()
    }
}

/// Depth stencil view description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_DEPTH_STENCIL_VIEW_DESC);

mod get_desc;
