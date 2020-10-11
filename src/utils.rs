/// As strict raw handle.
pub trait AsStrictRawHandle: FromStrictRawHandle {
    fn as_strict_raw_handle(&self) -> Self::StrictRawHandle;
}

/// From strict raw handle.
pub trait FromStrictRawHandle {
    type StrictRawHandle;

    unsafe fn from_strict_raw_handle(handle: Self::StrictRawHandle) -> Self;
}

/// Into strict raw handle.
pub trait IntoStrictRawHandle: FromStrictRawHandle {
    fn into_strict_raw_handle(self) -> Self::StrictRawHandle;
}

/// Transparent.
pub trait Transparent {
    type Target;
}
