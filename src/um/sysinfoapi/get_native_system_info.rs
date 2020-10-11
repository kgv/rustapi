use super::SystemInformation;
use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;

/// Get native system information.
#[derive(FnOnce, TypedBuilder)]
pub struct GetNativeSystemInfo {
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    system_information: MaybeUninit<SystemInformation>,
}

impl FnOnce<()> for GetNativeSystemInfo {
    type Output = SystemInformation;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::sysinfoapi::GetNativeSystemInfo;

        #[allow(non_snake_case)]
        unsafe {
            let lpSystemInfo = self.system_information.as_mut_ptr() as _;
            GetNativeSystemInfo(lpSystemInfo);
            self.system_information.assume_init()
        }
    }
}
