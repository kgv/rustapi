use crate::r#macro::Transparent;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::um::d3d11::D3D11_SUBRESOURCE_DATA;

/// Subresource data.
#[derive(Clone, Deref, DerefMut, From, Into, Transparent)]
#[repr(transparent)]
pub struct SubresourceData(D3D11_SUBRESOURCE_DATA);
