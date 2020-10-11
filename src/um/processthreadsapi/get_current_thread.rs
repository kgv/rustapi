use crate::shared::ntdef::Handle;
use std::os::windows::io::FromRawHandle;

/// Get current thread.
pub struct GetCurrentThread;

impl FnOnce<()> for GetCurrentThread {
    type Output = Handle;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetCurrentThread;

        unsafe {
            let r#return = GetCurrentThread();
            Handle::from_raw_handle(r#return)
        }
    }
}
