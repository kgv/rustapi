use self::{
    create_class_linkage::CreateClassLinkage,
    create_depth_stencil_state::{CreateDepthStencilState, CreateDepthStencilStateBuilder},
    create_pixel_shader::{CreatePixelShader, CreatePixelShaderBuilder},
    create_rasterizer_state::{CreateRasterizerState, CreateRasterizerStateBuilder},
    create_render_target_view::{CreateRenderTargetView, CreateRenderTargetViewBuilder},
    create_sampler_state::{CreateSamplerState, CreateSamplerStateBuilder},
    create_shader_resource_view::{CreateShaderResourceView, CreateShaderResourceViewBuilder},
    create_texture_2d::{CreateTexture2D, CreateTexture2DBuilder},
    get_immediate_context::GetImmediateContext,
};
use crate::utils::Transparent;
use anyhow::Result;
use winapi::um::d3d11::{
    ID3D11ClassLinkage, ID3D11Device, ID3D11DeviceContext, D3D11_SUBRESOURCE_DATA,
};
use wio::com::ComPtr;

/// D3D11 device.
pub trait Device {
    fn get_immediate_context(&self) -> ComPtr<ID3D11DeviceContext>;

    fn create_class_linkage(&self) -> Result<ComPtr<ID3D11ClassLinkage>>;

    fn create_depth_stencil_state(
        &self,
    ) -> CreateDepthStencilStateBuilder<((ComPtr<ID3D11Device>,), ())>;

    fn create_pixel_shader(&self) -> CreatePixelShaderBuilder<((ComPtr<ID3D11Device>,), (), ())>;

    fn create_rasterizer_state(
        &self,
    ) -> CreateRasterizerStateBuilder<((ComPtr<ID3D11Device>,), ())>;

    fn create_render_target_view(
        &self,
    ) -> CreateRenderTargetViewBuilder<((ComPtr<ID3D11Device>,), (), ())>;

    fn create_sampler_state(&self) -> CreateSamplerStateBuilder<((ComPtr<ID3D11Device>,), ())>;

    fn create_shader_resource_view(
        &self,
    ) -> CreateShaderResourceViewBuilder<((ComPtr<ID3D11Device>,), (), ())>;

    fn create_texture_2d<'a, T>(
        &'a self,
    ) -> CreateTexture2DBuilder<((ComPtr<ID3D11Device>,), (), ()), T>
    where
        T: 'a + Transparent<Target = D3D11_SUBRESOURCE_DATA>;
}

impl Device for ComPtr<ID3D11Device> {
    fn get_immediate_context(&self) -> ComPtr<ID3D11DeviceContext> {
        GetImmediateContext::builder().device(self.clone()).build()()
    }

    fn create_class_linkage(&self) -> Result<ComPtr<ID3D11ClassLinkage>> {
        CreateClassLinkage::builder().device(self.clone()).build()()
    }

    fn create_depth_stencil_state(
        &self,
    ) -> CreateDepthStencilStateBuilder<((ComPtr<ID3D11Device>,), ())> {
        CreateDepthStencilState::builder().device(self.clone())
    }

    fn create_pixel_shader(&self) -> CreatePixelShaderBuilder<((ComPtr<ID3D11Device>,), (), ())> {
        CreatePixelShader::builder().device(self.clone())
    }

    fn create_rasterizer_state(
        &self,
    ) -> CreateRasterizerStateBuilder<((ComPtr<ID3D11Device>,), ())> {
        CreateRasterizerState::builder().device(self.clone())
    }

    fn create_render_target_view(
        &self,
    ) -> CreateRenderTargetViewBuilder<((ComPtr<ID3D11Device>,), (), ())> {
        CreateRenderTargetView::builder().device(self.clone())
    }

    fn create_sampler_state(&self) -> CreateSamplerStateBuilder<((ComPtr<ID3D11Device>,), ())> {
        CreateSamplerState::builder().device(self.clone())
    }

    fn create_shader_resource_view(
        &self,
    ) -> CreateShaderResourceViewBuilder<((ComPtr<ID3D11Device>,), (), ())> {
        CreateShaderResourceView::builder().device(self.clone())
    }

    fn create_texture_2d<'a, T>(
        &'a self,
    ) -> CreateTexture2DBuilder<((ComPtr<ID3D11Device>,), (), ()), T>
    where
        T: 'a + Transparent<Target = D3D11_SUBRESOURCE_DATA>,
    {
        CreateTexture2D::builder().device(self.clone())
    }
}

mod create_class_linkage;
mod create_depth_stencil_state;
mod create_pixel_shader;
mod create_rasterizer_state;
mod create_render_target_view;
mod create_sampler_state;
mod create_shader_resource_view;
mod create_texture_2d;
mod get_immediate_context;
