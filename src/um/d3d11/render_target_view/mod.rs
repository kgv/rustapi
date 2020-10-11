use derive_more::{Deref, DerefMut, From, Into};
use std::fmt::{self, Debug, Formatter};
use winapi::{
    shared::dxgiformat::DXGI_FORMAT,
    um::d3d11::{ID3D11RenderTargetView, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION},
};
use wio::com::ComPtr;

use self::get_desc::GetDesc;

/// Render target view.
pub trait RenderTargetView {
    fn get_description(&self) -> Description;
}

impl RenderTargetView for ComPtr<ID3D11RenderTargetView> {
    fn get_description(&self) -> Description {
        GetDesc::builder().render_target_view(self.clone()).build()()
    }
}

/// Render target view description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_RENDER_TARGET_VIEW_DESC);

impl Description {
    #[inline]
    pub fn format(&self) -> DXGI_FORMAT {
        self.0.Format
    }

    #[inline]
    pub fn view_dimension(&self) -> D3D11_RTV_DIMENSION {
        self.0.ViewDimension
    }
}

impl Debug for Description {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("Description")
            .field("format", &self.format())
            .field("view_dimension", &self.view_dimension())
            .finish()
    }
}

mod get_desc;
