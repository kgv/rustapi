use crate::shared::ntdef::Handle;
use anyhow::{ensure, Result};
use std::{
    io,
    mem::MaybeUninit,
    os::windows::io::{AsRawHandle, FromRawHandle},
    ptr::null_mut,
};
use typed_builder::TypedBuilder;
use winapi::{
    shared::minwindef::FALSE,
    um::{
        minwinbase::{LPTHREAD_START_ROUTINE, SECURITY_ATTRIBUTES},
        winnt::PROCESS_ALL_ACCESS,
    },
};

/// Create remote thread.
#[derive(TypedBuilder)]
pub struct CreateRemoteThread<'a> {
    process: &'a Handle,
    #[builder(setter(strip_option))]
    thread_attributes: Option<&'a mut SECURITY_ATTRIBUTES>,
    #[builder(setter(strip_option))]
    stack_size: Option<usize>,
    start_address: LPTHREAD_START_ROUTINE,
    #[builder(setter(strip_option))]
    parameter_address: Option<usize>,
    #[builder(setter(strip_option))]
    creation_flags: Option<u32>,
    #[builder(default, setter(skip))]
    thread_id: u32,
}

impl FnOnce<()> for CreateRemoteThread<'_> {
    type Output = Result<(u32, Handle)>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::CreateRemoteThread;

        #[allow(non_snake_case)]
        let hProcess = self.process.as_raw_handle();
        #[allow(non_snake_case)]
        let lpThreadAttributes = self
            .thread_attributes
            .map_or(null_mut(), |thread_attributes| thread_attributes as _);
        #[allow(non_snake_case)]
        let dwStackSize = self.stack_size.unwrap_or_default();
        #[allow(non_snake_case)]
        let lpStartAddress = self.start_address;
        #[allow(non_snake_case)]
        let lpParameter = self
            .parameter_address
            .map_or(null_mut(), |parameter_address| parameter_address as _);
        #[allow(non_snake_case)]
        let dwCreationFlags = self.creation_flags.unwrap_or_default();
        #[allow(non_snake_case)]
        let lpThreadId = &mut self.thread_id;
        let r#return = unsafe {
            CreateRemoteThread(
                hProcess,
                lpThreadAttributes,
                dwStackSize,
                lpStartAddress,
                lpParameter,
                dwCreationFlags,
                lpThreadId,
            )
        };
        ensure!(!r#return.is_null(), io::Error::last_os_error());
        Ok((self.thread_id, unsafe { Handle::from_raw_handle(r#return) }))
    }
}

/// Get current process.
pub struct GetCurrentProcess;

impl FnOnce<()> for GetCurrentProcess {
    type Output = Handle;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetCurrentProcess;

        let r#return = unsafe { GetCurrentProcess() };
        unsafe { Handle::from_raw_handle(r#return) }
    }
}

/// Get current thread.
pub struct GetCurrentThread;

impl FnOnce<()> for GetCurrentThread {
    type Output = Handle;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetCurrentThread;

        let r#return = unsafe { GetCurrentThread() };
        unsafe { Handle::from_raw_handle(r#return) }
    }
}

/// Get thread exit code.
#[derive(TypedBuilder)]
pub struct GetExitCodeThread<'a> {
    thread: &'a Handle,
    #[builder(default, setter(skip))]
    exit_code: u32,
}

impl FnOnce<()> for GetExitCodeThread<'_> {
    type Output = Result<u32>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetExitCodeThread;

        #[allow(non_snake_case)]
        let hThread = self.thread.as_raw_handle();
        #[allow(non_snake_case)]
        let lpExitCode = &mut self.exit_code;
        let r#return = unsafe { GetExitCodeThread(hThread, lpExitCode) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.exit_code)
    }
}

/// GetProcessId.
#[derive(TypedBuilder)]
pub struct GetProcessId<'a> {
    process: &'a Handle,
}

impl FnOnce<()> for GetProcessId<'_> {
    type Output = u32;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::GetProcessId;

        #[allow(non_snake_case)]
        let Process = self.process.as_raw_handle();
        unsafe { GetProcessId(Process) }
    }
}

/// Set thread affinity mask.
#[derive(TypedBuilder)]
pub struct SetThreadAffinityMask<'a> {
    thread: &'a Handle,
    #[builder(default, setter(skip))]
    thread_affinity_mask: usize,
}

impl FnOnce<()> for SetThreadAffinityMask<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::winbase::SetThreadAffinityMask;

        #[allow(non_snake_case)]
        let hThread = self.thread.as_raw_handle();
        #[allow(non_snake_case)]
        let dwThreadAffinityMask = self.thread_affinity_mask;
        let r#return = unsafe { SetThreadAffinityMask(hThread, dwThreadAffinityMask) };
        ensure!(r#return != 0, io::Error::last_os_error());
        Ok(r#return)
    }
}

/// Open process.
#[derive(TypedBuilder)]
pub struct OpenProcess {
    #[builder(default = PROCESS_ALL_ACCESS)]
    desired_access: u32,
    #[builder(default)]
    inherit_handle: bool,
    id: u32,
}

impl FnOnce<()> for OpenProcess {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::OpenProcess;

        #[allow(non_snake_case)]
        let dwDesiredAccess = self.desired_access;
        #[allow(non_snake_case)]
        let bInheritHandle = self.inherit_handle as _;
        #[allow(non_snake_case)]
        let dwProcessId = self.id;
        let r#return = unsafe { OpenProcess(dwDesiredAccess, bInheritHandle, dwProcessId) };
        ensure!(!r#return.is_null(), io::Error::last_os_error());
        Ok(unsafe { Handle::from_raw_handle(r#return) })
    }
}

/// Open process token.
#[derive(TypedBuilder)]
pub struct OpenProcessToken<'a> {
    process: &'a Handle,
    desired_access: u32,
    #[builder(default = MaybeUninit::uninit(), setter(skip))]
    token_handle: MaybeUninit<Handle>,
}

impl FnOnce<()> for OpenProcessToken<'_> {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::processthreadsapi::OpenProcessToken;

        #[allow(non_snake_case)]
        let ProcessHandle = self.process.as_raw_handle();
        #[allow(non_snake_case)]
        let DesiredAccess = self.desired_access;
        #[allow(non_snake_case)]
        let TokenHandle = self.token_handle.as_mut_ptr() as _;
        let r#return = unsafe { OpenProcessToken(ProcessHandle, DesiredAccess, TokenHandle) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.token_handle.assume_init() })
    }
}

#[cfg(test)]
mod tests {
    use super::{GetCurrentProcess, OpenProcessToken};
    use anyhow::Result;
    use winapi::um::winnt::TOKEN_ADJUST_PRIVILEGES;

    #[test]
    fn open_process_token() -> Result<()> {
        let process = GetCurrentProcess();
        let _token = OpenProcessToken::builder()
            .process(&process)
            .desired_access(TOKEN_ADJUST_PRIVILEGES)
            .build()()?;
        Ok(())
    }
}
