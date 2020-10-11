use self::get_device::GetDevice;
use anyhow::Result;
use winapi::{
    shared::dxgi::{IDXGIDeviceSubObject, IDXGISwapChain},
    Interface,
};
use wio::com::ComPtr;

/// Device sub-object.
pub trait DeviceSubObject {
    fn device<T: Interface>(&self) -> Result<ComPtr<T>>;
}

impl DeviceSubObject for ComPtr<IDXGIDeviceSubObject> {
    fn device<T: Interface>(&self) -> Result<ComPtr<T>> {
        GetDevice::builder().device_sub_object(self.clone()).build()()
    }
}

impl DeviceSubObject for ComPtr<IDXGISwapChain> {
    fn device<T: Interface>(&self) -> Result<ComPtr<T>> {
        GetDevice::builder()
            .device_sub_object(self.cast().unwrap())
            .build()()
    }
}

mod get_device;
