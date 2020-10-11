use crate::um::{
    handleapi::{duplicate_handle, DuplicateHandle, DuplicateHandleBuilder},
    processthreadsapi::GetCurrentProcess,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut, Display, From, Into};
use rustapi_macro::Handle;
use std::{
    fmt::{self, Debug, Formatter},
    io,
    os::windows::io::RawHandle,
};
use winapi::{
    shared::{minwindef::FALSE, ntdef::LUID},
    um::{handleapi::CloseHandle, winnt::DUPLICATE_SAME_ACCESS},
};

/// Handle.
#[derive(Clone, Debug, Display, Handle)]
#[display(fmt = "{:#p}", _0)]
#[repr(transparent)]
pub struct Handle(RawHandle);

impl Handle {
    pub fn duplicate(&self) -> DuplicateHandleBuilder<((), (&Handle,), (), (), (), ())> {
        duplicate_handle().source_handle(self)
    }

    pub fn duplicate_for_current_process(&self) -> Result<Handle> {
        let process = GetCurrentProcess();
        DuplicateHandle::builder()
            .source_handle(self)
            .source_process_handle(&process)
            .target_process_handle(&process)
            .options(DUPLICATE_SAME_ACCESS)
            .build()()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let r#return = unsafe { CloseHandle(self.0) };
        assert!(r#return != FALSE, io::Error::last_os_error());
    }
}

unsafe impl Send for Handle {}

unsafe impl Sync for Handle {}

/// Locally unique identifier.
#[derive(Clone, Copy, Deref, DerefMut, Display, From, Into)]
#[display(fmt = "{:#x}", "i64::from(*self)")]
#[repr(transparent)]
pub struct Luid(LUID);

impl Luid {
    #[inline]
    pub fn high_part(&self) -> i32 {
        self.0.HighPart
    }

    #[inline]
    pub fn low_part(&self) -> u32 {
        self.0.LowPart
    }
}

impl Debug for Luid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Luid")
            .field("high_part", &self.high_part())
            .field("low_part", &self.low_part())
            .finish()
    }
}

impl From<Luid> for i64 {
    fn from(from: Luid) -> i64 {
        let high = from.high_part() as i64;
        let low = from.low_part() as i64;
        (high << 32) + low
    }
}

impl From<i64> for Luid {
    fn from(from: i64) -> Self {
        let low = (from & 0xffffffff) as _;
        let high = (from >> 32) as _;
        Luid(LUID {
            LowPart: low,
            HighPart: high,
        })
    }
}
