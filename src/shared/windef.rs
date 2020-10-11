use derive_more::{Deref, Display, From, Into};
use rustapi_macro::Handle;
use std::fmt::{self, Debug, Formatter};
use winapi::shared::windef::HWND;

/// Window handle.
#[derive(Clone, Copy, Deref, Display, From, Handle, Into)]
#[display(fmt = "{:#p}", _0)]
#[repr(transparent)]
pub struct WindowHandle(HWND);

impl Debug for WindowHandle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("WindowHandle")
            .field(&format_args!("{:#p}", self.0))
            .finish()
    }
}
