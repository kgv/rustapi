use crate::r#macro::FnOnce;
use std::ptr::{null, null_mut};
use typed_builder::TypedBuilder;
use winapi::um::d3d11::{
    ID3D11DepthStencilView, ID3D11DeviceContext, ID3D11RenderTargetView,
    D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT,
};
use wio::com::ComPtr;

/// Set render targets.
#[derive(FnOnce, TypedBuilder)]
pub struct OMSetRenderTargets<'a> {
    device_context: ComPtr<ID3D11DeviceContext>,
    #[builder(default)]
    render_target_views: &'a [ComPtr<ID3D11RenderTargetView>],
    #[builder(default)]
    depth_stencil_view: Option<ComPtr<ID3D11DepthStencilView>>,
}

impl FnOnce<()> for OMSetRenderTargets<'_> {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let NumViews = self.render_target_views.len() as _;
            assert!(NumViews <= D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT);
            let ppRenderTargetViews = if self.render_target_views.is_empty() {
                null()
            } else {
                self.render_target_views.as_ptr() as _
            };
            let ppDepthStencilView = self
                .depth_stencil_view
                .as_ref()
                .map_or(null_mut(), |v| v.as_raw() as _);
            self.device_context.OMSetRenderTargets(
                NumViews,
                ppRenderTargetViews,
                ppDepthStencilView,
            );
        }
    }
}
