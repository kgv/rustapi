use crate::{r#macro::FnOnce, shared::minwindef::ModuleHandle, utils::AsStrictRawHandle};
use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Disable thread library calls.
#[derive(FnOnce, TypedBuilder)]
pub struct DisableThreadLibraryCalls {
    #[builder(setter(into))]
    handle: ModuleHandle,
}

impl FnOnce<()> for DisableThreadLibraryCalls {
    type Output = Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::libloaderapi::DisableThreadLibraryCalls;

        #[allow(non_snake_case)]
        unsafe {
            let hLibModule = self.handle.as_strict_raw_handle();
            let r#return = DisableThreadLibraryCalls(hLibModule);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(())
        }
    }
}
