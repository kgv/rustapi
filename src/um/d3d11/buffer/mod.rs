use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::{ID3D11Buffer, D3D11_BUFFER_DESC, D3D11_USAGE};
use wio::com::ComPtr;

/// Buffer.
pub trait Buffer {
    fn get_description(&self) -> Description;
}

impl Buffer for ComPtr<ID3D11Buffer> {
    fn get_description(&self) -> Description {
        GetDesc::builder().buffer(self.clone()).build()()
    }
}

/// Buffer description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_BUFFER_DESC);

impl Description {
    #[inline]
    pub fn byte_width(&self) -> u32 {
        self.0.ByteWidth
    }

    #[inline]
    pub fn usage(&self) -> D3D11_USAGE {
        self.0.Usage
    }

    #[inline]
    pub fn bind_flags(&self) -> u32 {
        self.0.BindFlags
    }

    #[inline]
    pub fn cpu_access_flags(&self) -> u32 {
        self.0.CPUAccessFlags
    }

    #[inline]
    pub fn misc_flags(&self) -> u32 {
        self.0.MiscFlags
    }

    #[inline]
    pub fn structure_byte_stride(&self) -> u32 {
        self.0.StructureByteStride
    }
}

mod get_desc;
