use crate::r#macro::FnOnce;
use std::{mem::MaybeUninit, ptr::null_mut};
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{
    ID3D11DepthStencilView, ID3D11DeviceContext, ID3D11RenderTargetView,
    D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT,
};
use wio::com::ComPtr;

/// Render target count.
const CAPACITY: usize = D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT as _;

/// Get render targets.
#[derive(FnOnce, TypedBuilder)]
pub struct OMGetRenderTargets {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default = D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT)]
    number_of_views: u32,
    #[builder(default = Vec::with_capacity(CAPACITY), setter(skip))]
    render_target_views: Vec<ComPtr<ID3D11RenderTargetView>>,
    #[builder(default = Some(MaybeUninit::zeroed()))]
    depth_stencil_view: Option<MaybeUninit<ComPtr<ID3D11DepthStencilView>>>,
}

impl FnOnce<()> for OMGetRenderTargets {
    type Output = (
        Vec<ComPtr<ID3D11RenderTargetView>>,
        Option<ComPtr<ID3D11DepthStencilView>>,
    );

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let NumViews = self.number_of_views;
            assert!(NumViews <= D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT);
            let ppRenderTargetViews = if self.number_of_views == 0 {
                null_mut()
            } else {
                self.render_target_views.as_mut_ptr() as _
            };
            let ppDepthStencilView = self
                .depth_stencil_view
                .as_mut()
                .map_or(null_mut(), |v| v.as_mut_ptr() as _);
            self.device_context.OMGetRenderTargets(
                NumViews,
                ppRenderTargetViews,
                ppDepthStencilView,
            );
            self.render_target_views.set_len(self.number_of_views as _);
            (
                self.render_target_views,
                self.depth_stencil_view.map(|v| v.assume_init()),
            )
        }
    }
}

// // Original render target.
// let (mut render_target_views, depth_stencil_view): (
//     [ID3D11RenderTargetView; D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT as _],
//     _,
// ) = unsafe {
//     let mut render_target_views = MaybeUninit::<
//         [ID3D11RenderTargetView; D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT as _],
//     >::uninit();
//     let mut depth_stencil_view = null_mut();
//     device_context.OMGetRenderTargets(
//         D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT,
//         &mut render_target_views.get_mut().as_mut_ptr(),
//         &mut depth_stencil_view,
//     );
//     (render_target_views.assume_init(), depth_stencil_view)
// };
