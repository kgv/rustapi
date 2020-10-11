use crate::{shared::windef::WindowHandle, utils::FromStrictRawHandle};

/// Get foreground window.
pub struct GetForegroundWindow;

impl FnOnce<()> for GetForegroundWindow {
    type Output = Option<WindowHandle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetForegroundWindow;

        unsafe {
            let r#return = GetForegroundWindow();
            (!r#return.is_null()).then_some(WindowHandle::from_strict_raw_handle(r#return))
        }
    }
}
