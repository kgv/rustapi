use derive_more::{Deref, DerefMut, Display, From, Into};
use rustapi_macro::Handle;
use std::fmt::{self, Debug, Formatter};
use winapi::um::wincontypes::{COORD, HPCON, INPUT_RECORD};

/// Coordinates.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct Coordinates(COORD);

impl Coordinates {
    pub fn new(x: i16, y: i16) -> Self {
        Coordinates(COORD { X: x, Y: y })
    }
}

/// Input record.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct InputRecord(INPUT_RECORD);

impl InputRecord {
    #[inline]
    pub fn event_type(&self) -> u16 {
        self.0.EventType
    }
}

// TODO:
impl Debug for InputRecord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("InputRecord").finish()
    }
}

/// Pseudo console handle.
#[derive(Clone, Copy, Display, From, Handle, Into)]
#[display(fmt = "{:#p}", _0)]
#[repr(transparent)]
pub struct PseudoConsoleHandle(HPCON);

impl Debug for PseudoConsoleHandle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("PseudoConsoleHandle")
            .field(&format_args!("{:#p}", self.0))
            .finish()
    }
}
