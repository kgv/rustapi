use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::{dxgi::IDXGIDeviceSubObject, winerror::SUCCEEDED},
    Interface,
};
use wio::com::ComPtr;

/// Get device.
#[derive(FnOnce, TypedBuilder)]
pub struct GetDevice<T: Interface> {
    device_sub_object: ComPtr<IDXGIDeviceSubObject>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    device: MaybeUninit<ComPtr<T>>,
}

impl<T: Interface> FnOnce<()> for GetDevice<T> {
    type Output = Result<ComPtr<T>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let riid = &T::uuidof();
            let ppDevice = self.device.as_mut_ptr() as _;
            let r#return = self.device_sub_object.GetDevice(riid, ppDevice);
            ensure!(
                SUCCEEDED(r#return),
                "The IDXGIDeviceSubObject::GetDevice call FAILED ({}).",
                r#return,
            );
            Ok(self.device.assume_init())
        }
    }
}
