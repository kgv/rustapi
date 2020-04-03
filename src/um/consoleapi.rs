use crate::{
    shared::ntdef::Handle,
    um::wincontypes::{Coordinates, PseudoConsoleHandle},
    utils::AsStrictRawHandle,
};
use anyhow::{ensure, Result};
use std::{io, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Allocate console.
pub struct AllocConsole;

impl FnOnce<()> for AllocConsole {
    type Output = Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::AllocConsole;

        let r#return = unsafe { AllocConsole() };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(())
    }
}

/// Close pseudo console.
#[derive(TypedBuilder)]
pub struct ClosePseudoConsole {
    handle: PseudoConsoleHandle,
}

impl FnOnce<()> for ClosePseudoConsole {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::ClosePseudoConsole;

        #[allow(non_snake_case)]
        let hPC = self.handle.as_strict_raw_handle();
        unsafe { ClosePseudoConsole(hPC) };
    }
}

// /// Create pseudo console.
// #[derive(TypedBuilder)]
// pub struct CreatePseudoConsole {
//     size: Coordinates,
//     input: Handle,
//     output: Handle,
//     #[builder(default, setter(skip))]
//     flags: u32,
//     handle: PseudoConsoleHandle,
// }

// impl FnOnce<()> for CreatePseudoConsole {
//     type Output = ();

//     extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
//         use winapi::um::consoleapi::CreatePseudoConsole;

//         let size = self.size;
//         #[allow(non_snake_case)]
//         let hInput = self.input;
//         #[allow(non_snake_case)]
//         let hOutput = self.output;
//         #[allow(non_snake_case)]
//         let dwFlags = self.flags;
//         #[allow(non_snake_case)]
//         let phPC = self.handle;
//         unsafe { CreatePseudoConsole(size, hInput, hOutput, dwFlags, phPC) };
//     }
// }

/// Get number of console input events.
#[derive(TypedBuilder)]
pub struct GetNumberOfConsoleInputEvents<'a> {
    handle: &'a Handle,
    #[builder(default, setter(skip))]
    number_of_events: u32,
}

impl FnOnce<()> for GetNumberOfConsoleInputEvents<'_> {
    type Output = Result<u32>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::GetNumberOfConsoleInputEvents;

        #[allow(non_snake_case)]
        let hConsoleInput = self.handle.as_raw_handle();
        #[allow(non_snake_case)]
        let lpcNumberOfEvents = &mut self.number_of_events;
        let r#return = unsafe { GetNumberOfConsoleInputEvents(hConsoleInput, lpcNumberOfEvents) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.number_of_events)
    }
}
