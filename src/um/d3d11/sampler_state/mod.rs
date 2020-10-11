use self::get_desc::GetDesc;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::{
    ID3D11SamplerState, D3D11_COMPARISON_NEVER, D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FLOAT32_MAX,
    D3D11_SAMPLER_DESC, D3D11_TEXTURE_ADDRESS_CLAMP,
};
use wio::com::ComPtr;

/// Sampler state.
pub trait SamplerState {
    fn get_description(&self) -> Description;
}

impl SamplerState for ComPtr<ID3D11SamplerState> {
    fn get_description(&self) -> Description {
        GetDesc::builder().sampler_state(self.clone()).build()()
    }
}

/// Sampler state description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(D3D11_SAMPLER_DESC);

impl Default for Description {
    fn default() -> Self {
        Self(D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
            MipLODBias: 0f32,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D11_COMPARISON_NEVER,
            BorderColor: [1f32; 4],
            MinLOD: -D3D11_FLOAT32_MAX,
            MaxLOD: D3D11_FLOAT32_MAX,
        })
    }
}

mod get_desc;
