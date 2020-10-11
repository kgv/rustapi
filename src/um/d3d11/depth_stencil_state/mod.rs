use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::{
    shared::minwindef::TRUE,
    um::d3d11::{
        ID3D11DepthStencilState, D3D11_COMPARISON_FUNC, D3D11_DEPTH_STENCILOP_DESC,
        D3D11_DEPTH_STENCIL_DESC, D3D11_DEPTH_WRITE_MASK,
    },
};
use wio::com::ComPtr;

/// Depth stencil state.
pub trait DepthStencilState {
    fn get_description(&self) -> Description;
}

impl DepthStencilState for ComPtr<ID3D11DepthStencilState> {
    fn get_description(&self) -> Description {
        GetDesc::builder().depth_stencil_state(self.clone()).build()()
    }
}

/// Depth stencil state description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_DEPTH_STENCIL_DESC);

impl Description {
    #[inline]
    pub fn depth_enable(&self) -> bool {
        self.0.DepthEnable == TRUE
    }

    #[inline]
    pub fn depth_write_mask(&self) -> D3D11_DEPTH_WRITE_MASK {
        self.0.DepthWriteMask
    }

    #[inline]
    pub fn depth_func(&self) -> D3D11_COMPARISON_FUNC {
        self.0.DepthFunc
    }

    #[inline]
    pub fn stencil_enable(&self) -> bool {
        self.0.StencilEnable == TRUE
    }

    #[inline]
    pub fn stencil_read_mask(&self) -> u8 {
        self.0.StencilReadMask
    }

    #[inline]
    pub fn stencil_write_mask(&self) -> u8 {
        self.0.StencilWriteMask
    }

    #[inline]
    pub fn front_face(&self) -> D3D11_DEPTH_STENCILOP_DESC {
        self.0.FrontFace
    }

    #[inline]
    pub fn back_face(&self) -> D3D11_DEPTH_STENCILOP_DESC {
        self.0.BackFace
    }
}

mod get_desc;
