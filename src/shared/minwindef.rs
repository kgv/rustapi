use derive_more::{Display, From, Into};
use rustapi_macro::Handle;
use std::fmt::{self, Debug, Formatter};
use winapi::shared::minwindef::HMODULE;

/// Module handle.
#[derive(Clone, Copy, Display, From, Handle, Into)]
#[display(fmt = "{:#p}", _0)]
#[repr(transparent)]
pub struct ModuleHandle(HMODULE);

impl Debug for ModuleHandle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("ModuleHandle")
            .field(&format_args!("{:#p}", self.0))
            .finish()
    }
}
