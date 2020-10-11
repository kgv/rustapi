pub(crate) use self::{
    create_remote_thread::{CreateRemoteThread, CreateRemoteThreadBuilder},
    get_current_process::GetCurrentProcess,
    get_current_thread::GetCurrentThread,
    get_exit_code_thread::{GetExitCodeThread, GetExitCodeThreadBuilder},
    get_process_id::{GetProcessId, GetProcessIdBuilder},
    open_process::{OpenProcess, OpenProcessBuilder},
    open_process_token::{OpenProcessToken, OpenProcessTokenBuilder},
    set_thread_affinity_mask::{SetThreadAffinityMask, SetThreadAffinityMaskBuilder},
};
use crate::shared::ntdef::Handle;

pub fn create_remote_thread<'a>() -> CreateRemoteThreadBuilder<'a, ((), (), (), (), (), ())> {
    CreateRemoteThread::builder()
}

pub fn get_current_process<'a>() -> Handle {
    GetCurrentProcess()
}

pub fn get_current_thread<'a>() -> Handle {
    GetCurrentThread()
}

pub fn get_exit_code_thread<'a>() -> GetExitCodeThreadBuilder<'a, ((),)> {
    GetExitCodeThread::builder()
}

pub fn get_process_id<'a>() -> GetProcessIdBuilder<'a, ((),)> {
    GetProcessId::builder()
}

pub fn open_process() -> OpenProcessBuilder<((), (), ())> {
    OpenProcess::builder()
}

pub fn open_process_token<'a>() -> OpenProcessTokenBuilder<'a, ((), ())> {
    OpenProcessToken::builder()
}

pub fn set_thread_affinity_mask<'a>() -> SetThreadAffinityMaskBuilder<'a, ((),)> {
    SetThreadAffinityMask::builder()
}

mod create_remote_thread;
mod get_current_process;
mod get_current_thread;
mod get_exit_code_thread;
mod get_process_id;
mod open_process;
mod open_process_token;
mod set_thread_affinity_mask;
