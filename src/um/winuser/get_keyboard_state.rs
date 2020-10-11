use crate::r#macro::FnOnce;
use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Get keyboard state.
#[derive(FnOnce, TypedBuilder)]
pub struct GetKeyboardState {
    #[builder(default = [0; 256], setter(skip))]
    key_states: [u8; 256],
}

impl FnOnce<()> for GetKeyboardState {
    type Output = Result<[u8; 256]>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetKeyboardState;

        #[allow(non_snake_case)]
        unsafe {
            let lpKeyState = self.key_states.as_mut_ptr();
            let r#return = GetKeyboardState(lpKeyState);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(self.key_states)
        }
    }
}
