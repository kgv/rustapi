use self::{
    get_buffer::{GetBuffer, GetBufferBuilder},
    get_desc::GetDesc,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut, From, Into};
use winapi::{
    shared::{
        dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT},
        dxgitype::{DXGI_MODE_DESC, DXGI_SAMPLE_DESC, DXGI_USAGE},
        minwindef::{BOOL, UINT},
        windef::HWND,
    },
    Interface,
};
use wio::com::ComPtr;

/// Swap chain.
pub trait SwapChain {
    fn buffer<T: Interface>(&self) -> GetBufferBuilder<((ComPtr<IDXGISwapChain>,), ()), T>;

    fn get_description(&self) -> Result<Description>;
}

impl SwapChain for ComPtr<IDXGISwapChain> {
    fn buffer<T: Interface>(&self) -> GetBufferBuilder<((ComPtr<IDXGISwapChain>,), ()), T> {
        GetBuffer::builder().swap_chain(self.clone())
    }

    fn get_description(&self) -> Result<Description> {
        GetDesc::builder().swap_chain(self.clone()).build()()
    }
}

mod get_buffer;
mod get_desc;

/// Swap chain description.
#[derive(Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Description(DXGI_SWAP_CHAIN_DESC);

impl Description {
    #[inline]
    pub fn buffer_description(&self) -> DXGI_MODE_DESC {
        self.0.BufferDesc
    }

    #[inline]
    pub fn sample_description(&self) -> DXGI_SAMPLE_DESC {
        self.0.SampleDesc
    }

    #[inline]
    pub fn buffer_usage(&self) -> DXGI_USAGE {
        self.0.BufferUsage
    }

    #[inline]
    pub fn buffer_count(&self) -> UINT {
        self.0.BufferCount
    }

    #[inline]
    pub fn output_window(&self) -> HWND {
        self.0.OutputWindow
    }

    #[inline]
    pub fn windowed(&self) -> BOOL {
        self.0.Windowed
    }

    #[inline]
    pub fn swap_effect(&self) -> DXGI_SWAP_EFFECT {
        self.0.SwapEffect
    }

    #[inline]
    pub fn flags(&self) -> UINT {
        self.0.Flags
    }
}
