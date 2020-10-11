use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::{ID3D11Texture2D, D3D11_TEXTURE2D_DESC};
use wio::com::ComPtr;

/// Texture 2D.
pub trait Texture2D {
    fn get_description(&self) -> Description;
}

impl Texture2D for ComPtr<ID3D11Texture2D> {
    fn get_description(&self) -> Description {
        GetDesc::builder().texture_2d(self.clone()).build()()
    }
}

/// Texture 2D description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_TEXTURE2D_DESC);

mod get_desc;
