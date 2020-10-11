use crate::{r#macro::FnOnce, wrap::key::Key};
use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;

/// Get async key state.
#[derive(FnOnce, TypedBuilder)]
pub struct GetAsyncKeyState {
    key: Key,
}

impl FnOnce<()> for GetAsyncKeyState {
    type Output = Result<(bool, bool)>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetAsyncKeyState;

        #[allow(non_snake_case)]
        unsafe {
            let vKey = self.key.into();
            let r#return = GetAsyncKeyState(vKey);
            ensure!(r#return != 0, io::Error::last_os_error());
            let down = r#return < 0;
            let toggled = r#return & 1 != 0;
            Ok((down, toggled))
        }
    }
}
