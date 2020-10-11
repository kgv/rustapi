use crate::{r#macro::FnOnce, shared::windef::WindowHandle, utils::AsStrictRawHandle};
use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Set foreground window.
#[derive(FnOnce, TypedBuilder)]
pub struct SetForegroundWindow {
    window_handle: WindowHandle,
}

impl FnOnce<()> for SetForegroundWindow {
    type Output = Result<bool>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::SetForegroundWindow;

        #[allow(non_snake_case)]
        unsafe {
            let hWnd = self.window_handle.as_strict_raw_handle();
            let r#return = SetForegroundWindow(hWnd);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(r#return != FALSE)
        }
    }
}
