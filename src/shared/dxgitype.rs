use derive_more::{Deref, DerefMut, From, Into};
use winapi::shared::{dxgiformat::DXGI_FORMAT, dxgitype::DXGI_MODE_DESC};

/// DXGI mode description.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(DXGI_MODE_DESC);

impl Description {
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.Width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.0.Height
    }

    #[inline]
    pub fn format(&self) -> DXGI_FORMAT {
        self.0.Format
    }

    #[inline]
    pub fn scanline_ordering(&self) -> DXGI_FORMAT {
        self.0.ScanlineOrdering
    }

    #[inline]
    pub fn scaling(&self) -> DXGI_FORMAT {
        self.0.Scaling
    }
}
