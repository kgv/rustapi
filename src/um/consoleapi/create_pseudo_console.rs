use crate::{
    r#macro::FnOnce,
    shared::ntdef::Handle,
    um::wincontypes::{Coordinates, PseudoConsoleHandle},
};
use anyhow::{ensure, Result};
use std::{io, mem::MaybeUninit, os::windows::io::AsRawHandle};
use typed_builder::TypedBuilder;
use winapi::shared::winerror::S_OK;

/// Create pseudo console.
#[derive(FnOnce, TypedBuilder)]
pub struct CreatePseudoConsole {
    size: Coordinates,
    input: Handle,
    output: Handle,
    #[builder(default)]
    flags: u32,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    handle: MaybeUninit<PseudoConsoleHandle>,
}

impl FnOnce<()> for CreatePseudoConsole {
    type Output = Result<PseudoConsoleHandle>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::CreatePseudoConsole;

        #[allow(non_snake_case)]
        unsafe {
            let size = *self.size;
            let hInput = self.input.as_raw_handle();
            let hOutput = self.output.as_raw_handle();
            let dwFlags = self.flags;
            let phPC = self.handle.as_mut_ptr() as _;
            let r#return = CreatePseudoConsole(size, hInput, hOutput, dwFlags, phPC);
            ensure!(r#return != S_OK, io::Error::from_raw_os_error(r#return));
            Ok(self.handle.assume_init())
        }
    }
}
