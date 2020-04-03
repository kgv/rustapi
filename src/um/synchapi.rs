//! Change notification
//! Console input
//! Event
//! Memory resource notification
//! Mutex
//! [Process](crate::um::process::Process)
//! Semaphore
//! [Thread](crate::um::thread::Thread)
//! Waitable timer

use crate::shared::ntdef::Handle;
use std::{io, os::windows::io::AsRawHandle, thread::JoinHandle};
use typed_builder::TypedBuilder;
use winapi::{
    shared::winerror::WAIT_TIMEOUT,
    um::winbase::{INFINITE, WAIT_ABANDONED, WAIT_FAILED, WAIT_OBJECT_0},
};

/// Wait.
pub trait Wait: AsRawHandle {}

impl Wait for Handle {}

impl<T> Wait for JoinHandle<T> {}

/// Wait for single object.
#[derive(TypedBuilder)]
pub struct WaitForSingleObject<T: Wait> {
    handle: T,
    #[builder(default = INFINITE)]
    milliseconds: u32,
}

impl<T: Wait> FnOnce<()> for WaitForSingleObject<T> {
    type Output = io::Result<()>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::synchapi::WaitForSingleObject;

        #[allow(non_snake_case)]
        let hHandle = self.handle.as_raw_handle();
        #[allow(non_snake_case)]
        let dwMilliseconds = self.milliseconds;
        let r#return = unsafe { WaitForSingleObject(hHandle, dwMilliseconds) };
        match r#return {
            WAIT_OBJECT_0 => Ok(()),
            WAIT_ABANDONED => Err(io::Error::from_raw_os_error(WAIT_ABANDONED as _)),
            WAIT_TIMEOUT => Err(io::Error::from_raw_os_error(WAIT_TIMEOUT as _)),
            WAIT_FAILED => Err(io::Error::last_os_error()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WaitForSingleObject;
    use anyhow::Result;
    use std::thread;

    #[test]
    fn with_handle() -> Result<()> {
        let handle = thread::spawn(|| {});
        WaitForSingleObject::builder().handle(handle).build()()?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "The wait operation timed out.")]
    fn with_handle_and_milliseconds() {
        let handle = thread::spawn(|| loop {});
        WaitForSingleObject::builder()
            .handle(handle)
            .milliseconds(100)
            .build()()
        .unwrap();
    }
}
