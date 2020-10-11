use crate::shared::ntdef::Handle;
use std::os::windows::io::FromRawHandle;

/// Get current process.
pub struct GetCurrentProcess;

impl FnOnce<()> for GetCurrentProcess {
    type Output = Handle;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetCurrentProcess;

        unsafe {
            let r#return = GetCurrentProcess();
            Handle::from_raw_handle(r#return)
        }
    }
}
