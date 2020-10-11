use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;
use winapi::{
    shared::{dxgi::IDXGIObject, winerror::SUCCEEDED},
    Interface,
};
use wio::com::ComPtr;

/// Get parent. 
#[derive(FnOnce, TypedBuilder)]
pub struct GetParent<T: Interface> {
    object: ComPtr<IDXGIObject>,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    parent: MaybeUninit<ComPtr<T>>,
}

impl<T: Interface> FnOnce<()> for GetParent<T> {
    type Output = Result<ComPtr<T>>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        #[allow(non_snake_case)]
        unsafe {
            let riid = &T::uuidof();
            let ppParent = self.parent.as_mut_ptr() as _;
            let r#return = self.object.GetParent(riid, ppParent);
            ensure!(
                SUCCEEDED(r#return),
                "The IDXGIObject::GetParent call FAILED ({}).",
                r#return,
            );
            Ok(self.parent.assume_init())
        }
    }
}
