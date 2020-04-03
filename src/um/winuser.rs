pub use self::key::Key;

use crate::{
    shared::windef::WindowHandle,
    utils::{AsStrictRawHandle, FromStrictRawHandle},
};
use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Get async key state.
#[derive(TypedBuilder)]
pub struct GetAsyncKeyState {
    key: Key,
}

impl FnOnce<()> for GetAsyncKeyState {
    type Output = Result<(bool, bool)>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetAsyncKeyState;

        #[allow(non_snake_case)]
        let vKey = self.key.into();
        let r#return = unsafe { GetAsyncKeyState(vKey) };
        ensure!(r#return != 0, io::Error::last_os_error());
        let down = r#return < 0;
        let toggled = r#return & 1 != 0;
        Ok((down, toggled))
    }
}

/// Get foreground window.
pub struct GetForegroundWindow;

impl FnOnce<()> for GetForegroundWindow {
    type Output = Option<WindowHandle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetForegroundWindow;

        let r#return = unsafe { GetForegroundWindow() };
        (!r#return.is_null()).then_some(unsafe { WindowHandle::from_strict_raw_handle(r#return) })
    }
}

/// Get keyboard state.
#[derive(TypedBuilder)]
pub struct GetKeyboardState {
    #[builder(default = [0; 256], setter(skip))]
    states: [u8; 256],
}

impl FnOnce<()> for GetKeyboardState {
    type Output = Result<[u8; 256]>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetKeyboardState;

        #[allow(non_snake_case)]
        let lpKeyState = self.states.as_mut_ptr();
        let r#return = unsafe { GetKeyboardState(lpKeyState) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.states)
    }
}

/// Get key state.
#[derive(TypedBuilder)]
pub struct GetKeyState {
    key: Key,
}

impl FnOnce<()> for GetKeyState {
    type Output = (bool, bool);

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetKeyState;

        #[allow(non_snake_case)]
        let nVirtKey = self.key.into();
        let r#return = unsafe { GetKeyState(nVirtKey) };
        let down = r#return < 0;
        let toggled = r#return & 1 != 0;
        (down, toggled)
    }
}

/// Set foreground window.
#[derive(TypedBuilder)]
pub struct SetForegroundWindow {
    handle: WindowHandle,
}

impl FnOnce<()> for SetForegroundWindow {
    type Output = Result<bool>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::SetForegroundWindow;

        #[allow(non_snake_case)]
        let hWnd = self.handle.as_strict_raw_handle();
        let r#return = unsafe { SetForegroundWindow(hWnd) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(r#return != FALSE)
    }
}

mod key;
