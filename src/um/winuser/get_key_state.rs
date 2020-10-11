use crate::{r#macro::FnOnce, wrap::key::Key};
use typed_builder::TypedBuilder;

/// Get key state.
#[derive(FnOnce, TypedBuilder)]
pub struct GetKeyState {
    virtual_key: Key,
}

impl FnOnce<()> for GetKeyState {
    type Output = (bool, bool);

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winuser::GetKeyState;

        #[allow(non_snake_case)]
        unsafe {
            let nVirtKey = self.virtual_key.into();
            let r#return = GetKeyState(nVirtKey);
            let down = r#return < 0;
            let toggled = r#return & 1 != 0;
            (down, toggled)
        }
    }
}
