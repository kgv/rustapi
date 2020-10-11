pub(crate) use self::create_device_and_swap_chain::{
    CreateDeviceAndSwapChain, CreateDeviceAndSwapChainBuilder,
};
pub use self::{
    blob::Blob, buffer::Buffer, depth_stencil_state::DepthStencilState,
    depth_stencil_view::DepthStencilView, device::Device, device_context::DeviceContext,
    rasterizer_state::RasterizerState, render_target_view::RenderTargetView,
    sampler_state::SamplerState, shader_resource_view::ShaderResourceView,
    subresource_data::SubresourceData, texture_2d::Texture2D,
};

pub fn create_device_and_swap_chain<'a>(
) -> CreateDeviceAndSwapChainBuilder<'a, ((), (), (), (), (), (), (), (), ())> {
    CreateDeviceAndSwapChain::builder()
}

/// COM wrapper.
pub trait Com<T> {
    /// Constructs a wrapper from a raw pointer. It takes ownership.
    ///
    /// Note: It does __not__ call `AddRef`.
    unsafe fn from_raw(pointer: *mut T) -> Self;

    /// Returns a wrapped raw pointer. It consumes the wrapper and takes
    /// ownership.
    ///
    /// Note: It does __not__ call `Release`.
    fn into_raw(self) -> *mut T;

    /// Returns a wrapped raw pointer. It does not consume the wrapper and does
    /// not take ownership.
    fn as_raw(&self) -> *mut T;
}

mod blob;
mod buffer;
mod create_device_and_swap_chain;
mod depth_stencil_state;
mod depth_stencil_view;
mod device;
mod device_context;
mod rasterizer_state;
mod render_target_view;
mod sampler_state;
mod shader_resource_view;
mod subresource_data;
mod texture_2d;

// mod class_linkage;
// mod pixel_shader;
// mod resource;
// mod effect;
