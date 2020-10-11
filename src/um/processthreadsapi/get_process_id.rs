use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use std::os::windows::io::AsRawHandle;
use typed_builder::TypedBuilder;

/// GetProcessId.
#[derive(FnOnce, TypedBuilder)]
pub struct GetProcessId<'a> {
    process: &'a Handle,
}

impl FnOnce<()> for GetProcessId<'_> {
    type Output = u32;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetProcessId;

        #[allow(non_snake_case)]
        unsafe {
            let Process = self.process.as_raw_handle();
            GetProcessId(Process)
        }
    }
}
