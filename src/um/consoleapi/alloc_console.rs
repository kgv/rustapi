use anyhow::{ensure, Result};
use std::io;
use winapi::shared::minwindef::FALSE;

/// Allocate console.
pub struct AllocConsole;

impl FnOnce<()> for AllocConsole {
    type Output = Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::AllocConsole;

        unsafe {
            let r#return = AllocConsole();
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(())
        }
    }
}
