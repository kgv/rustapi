use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Get number of console input events.
#[derive(FnOnce, TypedBuilder)]
pub struct GetNumberOfConsoleInputEvents<'a> {
    console_input: &'a Handle,
    #[builder(default, setter(skip))]
    number_of_events: u32,
}

impl FnOnce<()> for GetNumberOfConsoleInputEvents<'_> {
    type Output = Result<u32>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::GetNumberOfConsoleInputEvents;

        #[allow(non_snake_case)]
        unsafe {
            let hConsoleInput = self.console_input.as_raw_handle();
            let lpcNumberOfEvents = &mut self.number_of_events;
            let r#return = GetNumberOfConsoleInputEvents(hConsoleInput, lpcNumberOfEvents);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.number_of_events)
        }
    }
}
