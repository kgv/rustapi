use crate::{
    r#macro::FnOnce,
    shared::{ntdef::Handle, windef::WindowHandle},
    um::wincontypes::InputRecord,
    utils::FromStrictRawHandle,
};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Free console.
pub struct FreeConsole;

impl FnOnce<()> for FreeConsole {
    type Output = Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::wincon::FreeConsole;

        let r#return = unsafe { FreeConsole() };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(())
    }
}

/// Get console window.
pub struct GetConsoleWindow;

impl FnOnce<()> for GetConsoleWindow {
    type Output = Option<WindowHandle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::wincon::GetConsoleWindow;

        let r#return = unsafe { GetConsoleWindow() };
        (!r#return.is_null()).then_some(unsafe { WindowHandle::from_strict_raw_handle(r#return) })
    }
}

/// Get number of console mouse buttons.
#[derive(FnOnce, TypedBuilder)]
pub struct GetNumberOfConsoleMouseButtons {
    #[builder(default, setter(skip))]
    number_of_mouse_buttons: u32,
}

impl FnOnce<()> for GetNumberOfConsoleMouseButtons {
    type Output = Result<u32>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::wincon::GetNumberOfConsoleMouseButtons;

        #[allow(non_snake_case)]
        let lpNumberOfMouseButtons = &mut self.number_of_mouse_buttons;
        let r#return = unsafe { GetNumberOfConsoleMouseButtons(lpNumberOfMouseButtons) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.number_of_mouse_buttons)
    }
}

/// Write console input.
#[derive(FnOnce, TypedBuilder)]
pub struct WriteConsoleInput<'a> {
    console_input: &'a Handle,
    buffer: &'a [InputRecord],
    #[builder(default, setter(skip))]
    number_of_events_written: u32,
}

impl FnOnce<()> for WriteConsoleInput<'_> {
    type Output = Result<u32>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::wincon::WriteConsoleInputW;

        if self.buffer.len() == 0 {
            return Ok(0);
        }

        #[allow(non_snake_case)]
        let hConsoleInput = self.console_input.as_raw_handle();
        #[allow(non_snake_case)]
        let lpBuffer = self.buffer.as_ptr() as _;
        #[allow(non_snake_case)]
        let nLength = self.buffer.len() as _;
        #[allow(non_snake_case)]
        let lpNumberOfEventsWritten = &mut self.number_of_events_written;
        let r#return = unsafe {
            WriteConsoleInputW(hConsoleInput, lpBuffer, nLength, lpNumberOfEventsWritten)
        };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.number_of_events_written)
    }
}

#[cfg(test)]
mod test {
    use super::WriteConsoleInput;

    mod write_console_input {
        use super::WriteConsoleInput;
        use crate::shared::ntdef::Handle;
        use std::os::windows::io::{FromRawHandle, IntoRawHandle};

        #[test]
        fn test() {
            let handle = unsafe { Handle::from_raw_handle(0 as _) };
            let buffer = &[];
            let write_console_input = WriteConsoleInput::builder()
                .console_input(&handle)
                .buffer(buffer)
                .build();
            assert_eq!(0, write_console_input.buffer.len());
            let _ = handle.into_raw_handle();
        }
    }
}
