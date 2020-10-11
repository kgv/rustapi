pub(crate) use self::{
    close_handle::{CloseHandle, CloseHandleBuilder},
    duplicate_handle::{DuplicateHandle, DuplicateHandleBuilder},
};

pub fn close_handle<'a>() -> CloseHandleBuilder<'a, ((),)> {
    CloseHandle::builder()
}

pub fn duplicate_handle<'a>() -> DuplicateHandleBuilder<'a, ((), (), (), (), (), ())> {
    DuplicateHandle::builder()
}

mod close_handle;
mod duplicate_handle;
