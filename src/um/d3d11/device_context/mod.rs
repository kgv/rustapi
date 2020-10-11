use self::{
    input_assembler::ia_get_vertex_buffers::IAGetVertexBuffersBuilder,
    output_merger::{
        om_get_render_targets::OMGetRenderTargetsBuilder,
        om_set_depth_stencil_state::OMSetDepthStencilStateBuilder,
        om_set_render_targets::OMSetRenderTargetsBuilder,
    },
    pixel_shader::{
        ps_set_samplers::PSSetSamplersBuilder, ps_set_shader::PSSetShaderBuilder,
        ps_set_shader_resources::PSSetShaderResourcesBuilder,
    },
    rasterizer::rs_set_state::RSSetStateBuilder,
    vertex_shader::vs_get_constant_buffers::VSGetConstantBuffersBuilder,
};
pub use self::{
    input_assembler::{
        ia_get_index_buffer::IAGetIndexBuffer, ia_get_vertex_buffers::IAGetVertexBuffers,
    },
    output_merger::{
        om_get_depth_stencil_state::OMGetDepthStencilState,
        om_get_render_targets::OMGetRenderTargets,
        om_set_depth_stencil_state::OMSetDepthStencilState,
        om_set_render_targets::OMSetRenderTargets,
    },
    pixel_shader::{
        ps_set_samplers::PSSetSamplers, ps_set_shader::PSSetShader,
        ps_set_shader_resources::PSSetShaderResources,
    },
    rasterizer::rs_set_state::RSSetState,
    vertex_shader::vs_get_constant_buffers::VSGetConstantBuffers,
};
use winapi::{
    shared::dxgiformat::DXGI_FORMAT,
    um::d3d11::{ID3D11Buffer, ID3D11DepthStencilState, ID3D11DeviceContext},
};
use wio::com::ComPtr;

/// Device context.
pub trait DeviceContext = InputAssembler + OutputMerger + PixelShader + Rasterizer + VertexShader;

/// Input assembler.
pub trait InputAssembler {
    fn get_index_buffer(&self) -> (ComPtr<ID3D11Buffer>, DXGI_FORMAT, u32);

    fn get_vertex_buffers(
        &self,
    ) -> IAGetVertexBuffersBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;
}

impl InputAssembler for ComPtr<ID3D11DeviceContext> {
    fn get_index_buffer(&self) -> (ComPtr<ID3D11Buffer>, DXGI_FORMAT, u32) {
        IAGetIndexBuffer::builder()
            .device_context(self.clone())
            .build()()
    }

    fn get_vertex_buffers(
        &self,
    ) -> IAGetVertexBuffersBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        IAGetVertexBuffers::builder().device_context(self.clone())
    }
}

/// Output merger.
pub trait OutputMerger {
    fn get_depth_stencil_state(&self) -> (ComPtr<ID3D11DepthStencilState>, u32);

    fn get_render_targets(
        &self,
    ) -> OMGetRenderTargetsBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;

    fn set_depth_stencil_state(
        &self,
    ) -> OMSetDepthStencilStateBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;

    fn set_render_targets(
        &self,
    ) -> OMSetRenderTargetsBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;
}

impl OutputMerger for ComPtr<ID3D11DeviceContext> {
    fn get_depth_stencil_state(&self) -> (ComPtr<ID3D11DepthStencilState>, u32) {
        OMGetDepthStencilState::builder()
            .device_context(self.clone())
            .build()()
    }

    fn get_render_targets(
        &self,
    ) -> OMGetRenderTargetsBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        OMGetRenderTargets::builder().device_context(self.clone())
    }

    fn set_depth_stencil_state(
        &self,
    ) -> OMSetDepthStencilStateBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        OMSetDepthStencilState::builder().device_context(self.clone())
    }

    fn set_render_targets(
        &self,
    ) -> OMSetRenderTargetsBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        OMSetRenderTargets::builder().device_context(self.clone())
    }
}

/// Pixel shader.
pub trait PixelShader {
    fn set_samplers(&self) -> PSSetSamplersBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;

    fn set_shader(&self) -> PSSetShaderBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;

    fn set_shader_resources(
        &self,
    ) -> PSSetShaderResourcesBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;
}

impl PixelShader for ComPtr<ID3D11DeviceContext> {
    fn set_samplers(&self) -> PSSetSamplersBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        PSSetSamplers::builder().device_context(self.clone())
    }

    fn set_shader(&self) -> PSSetShaderBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        PSSetShader::builder().device_context(self.clone())
    }

    fn set_shader_resources(
        &self,
    ) -> PSSetShaderResourcesBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        PSSetShaderResources::builder().device_context(self.clone())
    }
}

/// Rasterizer.
pub trait Rasterizer {
    fn set_state(&self) -> RSSetStateBuilder<((ComPtr<ID3D11DeviceContext>,), ())>;
}

impl Rasterizer for ComPtr<ID3D11DeviceContext> {
    fn set_state(&self) -> RSSetStateBuilder<((ComPtr<ID3D11DeviceContext>,), ())> {
        RSSetState::builder().device_context(self.clone())
    }
}

/// Vertex shader.
pub trait VertexShader {
    fn get_constant_buffers(
        &self,
    ) -> VSGetConstantBuffersBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())>;
}

impl VertexShader for ComPtr<ID3D11DeviceContext> {
    fn get_constant_buffers(
        &self,
    ) -> VSGetConstantBuffersBuilder<((ComPtr<ID3D11DeviceContext>,), (), ())> {
        VSGetConstantBuffers::builder().device_context(self.clone())
    }
}

mod input_assembler;
mod output_merger;
mod pixel_shader;
mod rasterizer;
mod vertex_shader;
