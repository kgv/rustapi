pub(crate) use self::{
    read_process_memory::{ReadProcessMemory, ReadProcessMemoryBuilder},
    virtual_alloc::{VirtualAlloc, VirtualAllocBuilder},
    virtual_query::{VirtualQuery, VirtualQueryBuilder},
    write_process_memory::{WriteProcessMemory, WriteProcessMemoryBuilder},
};

pub fn read_process_memory<'a>() -> ReadProcessMemoryBuilder<'a, ((), (), ())> {
    ReadProcessMemory::builder()
}

pub fn virtual_alloc() -> VirtualAllocBuilder<((), (), (), ())> {
    VirtualAlloc::builder()
}

pub fn virtual_query() -> VirtualQueryBuilder<((),)> {
    VirtualQuery::builder()
}

pub fn write_process_memory<'a>() -> WriteProcessMemoryBuilder<'a, ((), (), ())> {
    WriteProcessMemory::builder()
}

mod read_process_memory;
mod virtual_alloc;
mod virtual_query;
mod write_process_memory;
