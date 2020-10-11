pub(crate) use self::{
    get_async_key_state::{GetAsyncKeyState, GetAsyncKeyStateBuilder},
    get_foreground_window::GetForegroundWindow,
    get_key_state::{GetKeyState, GetKeyStateBuilder},
    get_keyboard_state::{GetKeyboardState, GetKeyboardStateBuilder},
    set_foreground_window::{SetForegroundWindow, SetForegroundWindowBuilder},
};
use crate::shared::windef::WindowHandle;

pub fn get_async_key_state() -> GetAsyncKeyStateBuilder<((),)> {
    GetAsyncKeyState::builder()
}

pub fn get_foreground_window() -> Option<WindowHandle> {
    GetForegroundWindow()
}

pub fn get_key_state() -> GetKeyStateBuilder<((),)> {
    GetKeyState::builder()
}

pub fn get_keyboard_state() -> GetKeyboardStateBuilder<()> {
    GetKeyboardState::builder()
}

pub fn set_foreground_window() -> SetForegroundWindowBuilder<((),)> {
    SetForegroundWindow::builder()
}

mod get_async_key_state;
mod get_foreground_window;
mod get_key_state;
mod get_keyboard_state;
mod set_foreground_window;
