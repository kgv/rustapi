use super::SystemInformation;
use crate::r#macro::FnOnce;
use std::mem::MaybeUninit;
use typed_builder::TypedBuilder;

/// Get system information.
#[derive(FnOnce, TypedBuilder)]
pub struct GetSystemInfo {
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    system_information: MaybeUninit<SystemInformation>,
}

impl Default for GetSystemInfo {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl FnOnce<()> for GetSystemInfo {
    type Output = SystemInformation;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::sysinfoapi::GetSystemInfo;

        #[allow(non_snake_case)]
        unsafe {
            let lpSystemInfo = self.system_information.as_mut_ptr() as _;
            GetSystemInfo(lpSystemInfo);
            self.system_information.assume_init()
        }
    }
}
