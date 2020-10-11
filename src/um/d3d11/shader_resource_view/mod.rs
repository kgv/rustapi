use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::{ID3D11ShaderResourceView, D3D11_SHADER_RESOURCE_VIEW_DESC};
use wio::com::ComPtr;

/// Shader resource view.
pub trait ShaderResourceView {
    fn get_description(&self) -> Description;
}

impl ShaderResourceView for ComPtr<ID3D11ShaderResourceView> {
    fn get_description(&self) -> Description {
        GetDesc::builder()
            .shader_resource_view(self.clone())
            .build()()
    }
}

/// Shader resource view description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_SHADER_RESOURCE_VIEW_DESC);

mod get_desc;
