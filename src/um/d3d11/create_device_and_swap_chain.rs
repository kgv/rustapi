use crate::{
    r#macro::FnOnce,
    shared::{dxgi::swap_chain::Description, minwindef::ModuleHandle},
    utils::AsStrictRawHandle,
};
use anyhow::{ensure, Result};
use std::{
    mem::MaybeUninit,
    ptr::{null, null_mut},
};
use typed_builder::TypedBuilder;
use winapi::{
    shared::{
        dxgi::{IDXGIAdapter, IDXGISwapChain},
        winerror::SUCCEEDED,
    },
    um::{
        d3d11::{ID3D11Device, ID3D11DeviceContext, D3D11_SDK_VERSION},
        d3dcommon::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL},
    },
};
use wio::com::ComPtr;

/// Direct 3D create device and swap chain.
#[derive(FnOnce, TypedBuilder)]
pub struct CreateDeviceAndSwapChain<'a> {
    #[builder(default, setter(strip_option))]
    adapter: Option<ComPtr<IDXGIAdapter>>,
    driver_type: D3D_DRIVER_TYPE,
    #[builder(default, setter(into))]
    software: Option<ModuleHandle>,
    #[builder(default)]
    flags: u32,
    #[builder(default, setter(strip_option))]
    feature_levels: Option<&'a [D3D_FEATURE_LEVEL]>,
    #[builder(default = D3D11_SDK_VERSION)]
    sdk_version: u32,
    #[builder(setter(into))]
    swap_chain_description: Description,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    swap_chain: MaybeUninit<ComPtr<IDXGISwapChain>>,
    #[builder(default = Some(MaybeUninit::zeroed()))]
    device: Option<MaybeUninit<ComPtr<ID3D11Device>>>,
    #[builder(default = Some(MaybeUninit::zeroed()), setter(skip))]
    feature_level: Option<MaybeUninit<D3D_FEATURE_LEVEL>>,
    #[builder(default = Some(MaybeUninit::zeroed()))]
    device_context: Option<MaybeUninit<ComPtr<ID3D11DeviceContext>>>,
}

impl FnOnce<()> for CreateDeviceAndSwapChain<'_> {
    type Output = Result<(
        ComPtr<IDXGISwapChain>,
        Option<ComPtr<ID3D11Device>>,
        Option<ComPtr<ID3D11DeviceContext>>,
    )>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::d3d11::D3D11CreateDeviceAndSwapChain;

        #[allow(non_snake_case)]
        unsafe {
            let pAdapter = self.adapter.map_or(null_mut(), |v| v.as_raw());
            let DriverType = self.driver_type;
            let Software = self
                .software
                .map_or(null_mut(), |v| v.as_strict_raw_handle());
            let Flags = self.flags;
            let pFeatureLevels = self.feature_levels.map_or(null(), |v| v.as_ptr());
            let FeatureLevels = self.feature_levels.map_or(0, |v| v.len() as _);
            let SDKVersion = self.sdk_version;
            let pSwapChainDesc = &*self.swap_chain_description;
            let ppSwapChain = self.swap_chain.as_mut_ptr() as _;
            let ppDevice = self
                .device
                .as_mut()
                .map_or(null_mut(), |v| v.as_mut_ptr() as _);
            let pFeatureLevel = self
                .feature_level
                .as_mut()
                .map_or(null_mut(), |v| v.as_mut_ptr());
            let ppImmediateContext = self
                .device_context
                .as_mut()
                .map_or(null_mut(), |v| v.as_mut_ptr() as _);
            let r#return = D3D11CreateDeviceAndSwapChain(
                pAdapter,
                DriverType,
                Software,
                Flags,
                pFeatureLevels,
                FeatureLevels,
                SDKVersion,
                pSwapChainDesc,
                ppSwapChain,
                ppDevice,
                pFeatureLevel,
                ppImmediateContext,
            );
            ensure!(
                SUCCEEDED(r#return),
                "The D3D11CreateDeviceAndSwapChain call FAILED ({:#x}).",
                r#return,
            );
            Ok((
                self.swap_chain.assume_init(),
                self.device.map(|v| v.assume_init()),
                self.device_context.map(|v| v.assume_init()),
            ))
        }
    }
}
