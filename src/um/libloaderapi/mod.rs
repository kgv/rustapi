pub(crate) use self::{
    disable_thread_library_calls::{DisableThreadLibraryCalls, DisableThreadLibraryCallsBuilder},
    get_module_handle::{GetModuleHandle, GetModuleHandleBuilder},
    get_proc_address::{GetProcAddress, GetProcAddressBuilder},
};

pub fn disable_thread_library_calls() -> DisableThreadLibraryCallsBuilder<((),)> {
    DisableThreadLibraryCalls::builder()
}

pub fn get_module_handle<'a>() -> GetModuleHandleBuilder<'a, ((),)> {
    GetModuleHandle::builder()
}

pub fn get_proc_address<'a>() -> GetProcAddressBuilder<'a, ((), ())> {
    GetProcAddress::builder()
}

mod disable_thread_library_calls;
mod get_module_handle;
mod get_proc_address;
