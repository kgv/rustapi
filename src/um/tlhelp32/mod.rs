use crate::{shared::minwindef::ModuleHandle, utils::FromStrictRawHandle};
use derive_more::{Deref, DerefMut, From, Into};
use partial_uninit::PartialUninit;
use std::{
    fmt::{self, Debug, Formatter},
    mem::size_of,
    path::PathBuf,
};
use widestring::WideCStr;
use winapi::um::tlhelp32::{HEAPLIST32, MODULEENTRY32W, PROCESSENTRY32W, THREADENTRY32};

pub(crate) use self::{
    create_toolhelp32_snapshot::{CreateToolhelp32Snapshot, CreateToolhelp32SnapshotBuilder},
    heap32_list_first::{Heap32ListFirst, Heap32ListFirstBuilder},
    heap32_list_next::{Heap32ListNext, Heap32ListNextBuilder},
    module32_first::{Module32First, Module32FirstBuilder},
    module32_next::{Module32Next, Module32NextBuilder},
    process32_first::{Process32First, Process32FirstBuilder},
    process32_next::{Process32Next, Process32NextBuilder},
    thread32_first::{Thread32First, Thread32FirstBuilder},
    thread32_next::{Thread32Next, Thread32NextBuilder},
};

pub fn create_toolhelp32_snapshot() -> CreateToolhelp32SnapshotBuilder<((), ())> {
    CreateToolhelp32Snapshot::builder()
}

pub fn heap_first<'a>() -> Heap32ListFirstBuilder<'a, ((),)> {
    Heap32ListFirst::builder()
}

pub fn heap_next<'a>() -> Heap32ListNextBuilder<'a, ((),)> {
    Heap32ListNext::builder()
}

pub fn module_first<'a>() -> Module32FirstBuilder<'a, ((),)> {
    Module32First::builder()
}

pub fn module_next<'a>() -> Module32NextBuilder<'a, ((),)> {
    Module32Next::builder()
}

pub fn process_first<'a>() -> Process32FirstBuilder<'a, ((),)> {
    Process32First::builder()
}

pub fn process_next<'a>() -> Process32NextBuilder<'a, ((),)> {
    Process32Next::builder()
}

pub fn thread_first<'a>() -> Thread32FirstBuilder<'a, ((),)> {
    Thread32First::builder()
}

pub fn thread_next<'a>() -> Thread32NextBuilder<'a, ((),)> {
    Thread32Next::builder()
}

mod create_toolhelp32_snapshot;
mod heap32_list_first;
mod heap32_list_next;
mod module32_first;
mod module32_next;
mod process32_first;
mod process32_next;
mod thread32_first;
mod thread32_next;

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
