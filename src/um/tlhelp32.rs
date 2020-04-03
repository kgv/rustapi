use crate::{
    shared::{minwindef::ModuleHandle, ntdef::Handle},
    utils::FromStrictRawHandle,
};
use anyhow::{ensure, Result};
use derive_more::{Deref, DerefMut, From, Into};
use partial_uninit::PartialUninit;
use std::{
    fmt::{self, Debug, Formatter},
    io,
    mem::{size_of, MaybeUninit},
    os::windows::io::{AsRawHandle, FromRawHandle},
    path::PathBuf,
};
use typed_builder::TypedBuilder;
use widestring::WideCStr;
use winapi::{
    shared::minwindef::FALSE,
    um::{
        handleapi::INVALID_HANDLE_VALUE,
        tlhelp32::{HEAPLIST32, MODULEENTRY32W, PROCESSENTRY32W, THREADENTRY32},
    },
};

/// Create snapshot.
#[derive(TypedBuilder)]
pub struct CreateToolhelp32Snapshot {
    flags: u32,
    #[builder(default)]
    process_id: u32,
}

impl FnOnce<()> for CreateToolhelp32Snapshot {
    type Output = Result<Handle>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::CreateToolhelp32Snapshot;

        #[allow(non_snake_case)]
        let dwFlags = self.flags;
        #[allow(non_snake_case)]
        let th32ProcessID = self.process_id;
        let r#return = unsafe { CreateToolhelp32Snapshot(dwFlags, th32ProcessID) };
        ensure!(r#return != INVALID_HANDLE_VALUE, io::Error::last_os_error());
        Ok(unsafe { Handle::from_raw_handle(r#return) })
    }
}

/// Heap entry.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct HeapEntry(HEAPLIST32);

impl HeapEntry {
    #[inline]
    pub fn size(&self) -> usize {
        self.0.dwSize
    }

    #[inline]
    pub fn process_id(&self) -> u32 {
        self.0.th32ProcessID
    }

    #[inline]
    pub fn heap_id(&self) -> usize {
        self.0.th32HeapID
    }

    #[inline]
    pub fn flags(&self) -> u32 {
        self.0.dwFlags
    }
}

impl Debug for HeapEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("HeapEntry")
            .field("size", &self.size())
            .field("process_id", &self.process_id())
            .field("heap_id", &self.heap_id())
            .field("flags", &self.flags())
            .finish()
    }
}

impl PartialUninit for HeapEntry {
    fn partial_init(&mut self) {
        self.0.dwSize = size_of::<HEAPLIST32>() as _;
    }
}

/// Heap first.
#[derive(TypedBuilder)]
pub struct Heap32ListFirst<'a> {
    snapshot: &'a Handle,
    #[builder(default = HeapEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<HeapEntry>,
}

impl FnOnce<()> for Heap32ListFirst<'_> {
    type Output = Result<HeapEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Heap32ListFirst;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Heap32ListFirst(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Heap first.
#[derive(TypedBuilder)]
pub struct Heap32ListNext<'a> {
    snapshot: &'a Handle,
    #[builder(default = HeapEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<HeapEntry>,
}

impl FnOnce<()> for Heap32ListNext<'_> {
    type Output = Result<HeapEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Heap32ListNext;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Heap32ListNext(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Module entry.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct ModuleEntry(MODULEENTRY32W);

impl ModuleEntry {
    #[inline]
    pub fn process_id(&self) -> u32 {
        self.0.th32ProcessID
    }

    #[inline]
    pub fn base_address(&self) -> usize {
        self.0.modBaseAddr as _
    }

    #[inline]
    pub fn base_size(&self) -> u32 {
        self.0.modBaseSize
    }

    #[inline]
    pub fn handle(&self) -> ModuleHandle {
        unsafe { ModuleHandle::from_strict_raw_handle(self.0.hModule) }
    }

    #[inline]
    pub fn name(&self) -> String {
        WideCStr::from_slice_with_nul(&self.szModule)
            .unwrap()
            .to_string_lossy()
    }

    #[inline]
    pub fn path(&self) -> PathBuf {
        WideCStr::from_slice_with_nul(&self.szExePath)
            .unwrap()
            .to_os_string()
            .into()
    }
}

impl Debug for ModuleEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ModuleEntry")
            .field("process_id", &self.process_id())
            .field("base_address", &self.base_address())
            .field("base_size", &self.base_size())
            .field("handle", &self.handle())
            .field("name", &self.name())
            .field("path", &self.path())
            .finish()
    }
}

impl PartialUninit for ModuleEntry {
    fn partial_init(&mut self) {
        self.0.dwSize = size_of::<MODULEENTRY32W>() as _;
    }
}

/// Module first.
#[derive(TypedBuilder)]
pub struct Module32First<'a> {
    snapshot: &'a Handle,
    #[builder(default = ModuleEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ModuleEntry>,
}

impl FnOnce<()> for Module32First<'_> {
    type Output = Result<ModuleEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Module32FirstW;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Module32FirstW(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Module next.
#[derive(TypedBuilder)]
pub struct Module32Next<'a> {
    snapshot: &'a Handle,
    #[builder(default = ModuleEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ModuleEntry>,
}

impl FnOnce<()> for Module32Next<'_> {
    type Output = Result<ModuleEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Module32NextW;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Module32NextW(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Process entry.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct ProcessEntry(PROCESSENTRY32W);

impl ProcessEntry {
    #[inline]
    pub fn process_id(&self) -> u32 {
        self.0.th32ProcessID
    }

    #[inline]
    pub fn parent_process_id(&self) -> u32 {
        self.0.th32ParentProcessID
    }

    #[inline]
    pub fn threads_count(&self) -> u32 {
        self.0.cntThreads
    }

    #[inline]
    pub fn base_priority(&self) -> i32 {
        self.0.pcPriClassBase
    }

    #[inline]
    pub fn name(&self) -> String {
        WideCStr::from_slice_with_nul(&self.szExeFile)
            .unwrap()
            .to_string_lossy()
    }
}

impl Debug for ProcessEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ProcessEntry")
            .field("process_id", &self.process_id())
            .field("parent_process_id", &self.parent_process_id())
            .field("threads_count", &self.threads_count())
            .field("base_priority", &self.base_priority())
            .field("name", &self.name())
            .finish()
    }
}

impl PartialUninit for ProcessEntry {
    fn partial_init(&mut self) {
        self.0.dwSize = size_of::<PROCESSENTRY32W>() as _;
    }
}

/// Process first.
#[derive(TypedBuilder)]
pub struct Process32First<'a> {
    snapshot: &'a Handle,
    #[builder(default = ProcessEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ProcessEntry>,
}

impl FnOnce<()> for Process32First<'_> {
    type Output = Result<ProcessEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Process32FirstW;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Process32FirstW(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Process next.
#[derive(TypedBuilder)]
pub struct Process32Next<'a> {
    snapshot: &'a Handle,
    #[builder(default = ProcessEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ProcessEntry>,
}

impl FnOnce<()> for Process32Next<'_> {
    type Output = Result<ProcessEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Process32NextW;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Process32NextW(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Thread entry.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct ThreadEntry(THREADENTRY32);

impl ThreadEntry {
    #[inline]
    pub fn size(&self) -> u32 {
        self.0.dwSize
    }

    #[inline]
    pub fn thread_id(&self) -> u32 {
        self.0.th32ThreadID
    }

    #[inline]
    pub fn process_id(&self) -> u32 {
        self.0.th32OwnerProcessID
    }

    #[inline]
    pub fn base_pri(&self) -> i32 {
        self.0.tpBasePri
    }

    #[inline]
    pub fn delta_pri(&self) -> i32 {
        self.0.tpDeltaPri
    }

    #[inline]
    pub fn flags(&self) -> u32 {
        self.0.dwFlags
    }
}

impl Debug for ThreadEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ThreadEntry")
            .field("size", &self.size())
            .field("thread_id", &self.thread_id())
            .field("process_id", &self.process_id())
            .field("base_pri", &self.base_pri())
            .field("delta_pri", &self.delta_pri())
            .field("flags", &self.flags())
            .finish()
    }
}

impl PartialUninit for ThreadEntry {
    fn partial_init(&mut self) {
        self.0.dwSize = size_of::<THREADENTRY32>() as _;
    }
}

/// Thread32 first.
#[derive(TypedBuilder)]
pub struct Thread32First<'a> {
    snapshot: &'a Handle,
    #[builder(default = ThreadEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ThreadEntry>,
}

impl FnOnce<()> for Thread32First<'_> {
    type Output = Result<ThreadEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Thread32First;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Thread32First(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}

/// Thread next.
#[derive(TypedBuilder)]
pub struct Thread32Next<'a> {
    snapshot: &'a Handle,
    #[builder(default = ThreadEntry::partial_uninit(), setter(skip))]
    entry: MaybeUninit<ThreadEntry>,
}

impl FnOnce<()> for Thread32Next<'_> {
    type Output = Result<ThreadEntry>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::tlhelp32::Thread32Next;

        #[allow(non_snake_case)]
        let hSnapshot = self.snapshot.as_raw_handle();
        #[allow(non_snake_case)]
        let lppe = self.entry.as_mut_ptr() as _;
        let r#return = unsafe { Thread32Next(hSnapshot, lppe) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(unsafe { self.entry.assume_init() })
    }
}
