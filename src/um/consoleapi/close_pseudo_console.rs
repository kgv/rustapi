use crate::{r#macro::FnOnce, um::wincontypes::PseudoConsoleHandle, utils::AsStrictRawHandle};
use typed_builder::TypedBuilder;

/// Close pseudo console.
#[derive(FnOnce, TypedBuilder)]
pub struct ClosePseudoConsole {
    handle: PseudoConsoleHandle,
}

impl FnOnce<()> for ClosePseudoConsole {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::consoleapi::ClosePseudoConsole;

        #[allow(non_snake_case)]
        unsafe {
            let hPC = self.handle.as_strict_raw_handle();
            ClosePseudoConsole(hPC);
        }
    }
}
