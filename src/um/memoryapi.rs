use crate::{shared::ntdef::Handle, um::winnt::MemoryBasicInformation};
use anyhow::{ensure, Result};
use std::{
    io,
    mem::{size_of, MaybeUninit},
    os::windows::io::AsRawHandle,
    ptr::NonNull,
};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Read process memory.
#[derive(TypedBuilder)]
pub struct ReadProcessMemory<'a> {
    process: &'a Handle,
    base_address: &'a usize,
    buffer: &'a mut [u8],
    #[builder(default, setter(skip))]
    number_of_bytes_read: usize,
}

impl FnOnce<()> for ReadProcessMemory<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::ReadProcessMemory;

        #[allow(non_snake_case)]
        let hProcess = self.process.as_raw_handle();
        #[allow(non_snake_case)]
        let lpBaseAddress = self.base_address as *const _ as _;
        #[allow(non_snake_case)]
        let lpBuffer = self.buffer.as_mut_ptr() as _;
        #[allow(non_snake_case)]
        let nSize = self.buffer.len();
        #[allow(non_snake_case)]
        let lpNumberOfBytesRead = &mut self.number_of_bytes_read;
        let r#return = unsafe {
            ReadProcessMemory(
                hProcess,
                lpBaseAddress,
                lpBuffer,
                nSize,
                lpNumberOfBytesRead,
            )
        };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.number_of_bytes_read)
    }
}

/// Virtual allocation.
#[derive(TypedBuilder)]
pub struct VirtualAlloc {
    #[builder(default)]
    address: usize,
    size: usize,
    allocation_type: u32,
    protect: u32,
}

impl FnOnce<()> for VirtualAlloc {
    type Output = Result<NonNull<u8>>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::VirtualAlloc;

        #[allow(non_snake_case)]
        let lpAddress = self.address as _;
        #[allow(non_snake_case)]
        let dwSize = self.size;
        #[allow(non_snake_case)]
        let flAllocationType = self.allocation_type;
        #[allow(non_snake_case)]
        let flProtect = self.protect;
        let r#return = unsafe { VirtualAlloc(lpAddress, dwSize, flAllocationType, flProtect) };
        ensure!(!r#return.is_null(), io::Error::last_os_error());
        Ok(unsafe { NonNull::new_unchecked(r#return as _) })
    }
}

impl<T, U, V, W> VirtualAllocBuilder<(T, U, V, W)> {
    pub fn process(
        self,
        process: &Handle,
    ) -> extended::VirtualAllocBuilder<((&Handle,), T, U, V, W)> {
        extended::VirtualAllocBuilder::from(self).process(process)
    }
}

/// Virtual query.
#[derive(TypedBuilder)]
pub struct VirtualQuery {
    address: usize,
    #[builder(default = MaybeUninit::uninit(), setter(skip))]
    memory_basic_information: MaybeUninit<MemoryBasicInformation>,
}

impl FnOnce<()> for VirtualQuery {
    type Output = Result<MemoryBasicInformation>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::VirtualQuery;

        #[allow(non_snake_case)]
        let lpAddress = self.address as _;
        #[allow(non_snake_case)]
        let lpBuffer = self.memory_basic_information.as_mut_ptr() as _;
        #[allow(non_snake_case)]
        let dwLength = size_of::<MemoryBasicInformation>();
        let r#return = unsafe { VirtualQuery(lpAddress, lpBuffer, dwLength) };
        ensure!(r#return != 0, io::Error::last_os_error());
        ensure!(
            r#return == size_of::<MemoryBasicInformation>(),
            "Invalid buffer size (expected: {}, found: {}).",
            size_of::<MemoryBasicInformation>(),
            r#return,
        );
        let information = unsafe { self.memory_basic_information.assume_init() };
        ensure!(
            information.base_address() == self.address,
            "Invalid base address (expected: {:x}, found: {:x}).",
            self.address,
            information.base_address(),
        );
        Ok(information)
    }
}

impl<T> VirtualQueryBuilder<(T,)> {
    pub fn process(self, process: &Handle) -> extended::VirtualQueryBuilder<'_, ((&Handle,), T)> {
        extended::VirtualQueryBuilder::from(self).process(process)
    }
}

/// Write process memory.
#[derive(TypedBuilder)]
pub struct WriteProcessMemory<'a> {
    process: &'a Handle,
    base_address: usize,
    buffer: &'a [u8],
    #[builder(default, setter(skip))]
    number_of_bytes_written: usize,
}

impl FnOnce<()> for WriteProcessMemory<'_> {
    type Output = Result<usize>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::WriteProcessMemory;

        #[allow(non_snake_case)]
        let hProcess = self.process.as_raw_handle();
        #[allow(non_snake_case)]
        let lpBaseAddress = self.base_address as _;
        #[allow(non_snake_case)]
        let lpBuffer = self.buffer.as_ptr() as _;
        #[allow(non_snake_case)]
        let nSize = self.buffer.len();
        #[allow(non_snake_case)]
        let lpNumberOfBytesWritten = &mut self.number_of_bytes_written;
        let r#return = unsafe {
            WriteProcessMemory(
                hProcess,
                lpBaseAddress,
                lpBuffer,
                nSize,
                lpNumberOfBytesWritten,
            )
        };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.number_of_bytes_written)
    }
}

pub mod extended {
    use crate::{shared::ntdef::Handle, um::winnt::MemoryBasicInformation};
    use anyhow::{ensure, Result};
    use std::{
        io,
        marker::PhantomData,
        mem::{size_of, MaybeUninit},
        os::windows::io::AsRawHandle,
        ptr::NonNull,
    };
    use typed_builder::TypedBuilder;

    /// Virtual allocation extended.
    #[derive(TypedBuilder)]
    pub struct VirtualAlloc<'a> {
        process: &'a Handle,
        #[builder(default)]
        address: usize,
        size: usize,
        allocation_type: u32,
        protect: u32,
    }

    impl FnOnce<()> for VirtualAlloc<'_> {
        type Output = Result<NonNull<u8>>;

        extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
            use winapi::um::memoryapi::VirtualAllocEx;

            #[allow(non_snake_case)]
            let hProcess = self.process.as_raw_handle();
            #[allow(non_snake_case)]
            let lpAddress = self.address as _;
            #[allow(non_snake_case)]
            let dwSize = self.size;
            #[allow(non_snake_case)]
            let flAllocationType = self.allocation_type;
            #[allow(non_snake_case)]
            let flProtect = self.protect;
            let r#return =
                unsafe { VirtualAllocEx(hProcess, lpAddress, dwSize, flAllocationType, flProtect) };
            ensure!(!r#return.is_null(), io::Error::last_os_error());
            Ok(unsafe { NonNull::new_unchecked(r#return as _) })
        }
    }

    impl<'a, T, U, V, W> From<super::VirtualAllocBuilder<(T, U, V, W)>>
        for VirtualAllocBuilder<'a, ((), T, U, V, W)>
    {
        fn from(from: super::VirtualAllocBuilder<(T, U, V, W)>) -> Self {
            Self {
                fields: (
                    (),
                    from.fields.0,
                    from.fields.1,
                    from.fields.2,
                    from.fields.3,
                ),
                _phantom: PhantomData,
            }
        }
    }

    /// Virtual query extended.
    #[derive(TypedBuilder)]
    pub struct VirtualQuery<'a> {
        process: &'a Handle,
        address: usize,
        #[builder(default = MaybeUninit::uninit(), setter(skip))]
        memory_basic_information: MaybeUninit<MemoryBasicInformation>,
    }

    impl FnOnce<()> for VirtualQuery<'_> {
        type Output = Result<MemoryBasicInformation>;

        extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
            use winapi::um::memoryapi::VirtualQueryEx;

            #[allow(non_snake_case)]
            let hProcess = self.process.as_raw_handle();
            #[allow(non_snake_case)]
            let lpAddress = self.address as _;
            #[allow(non_snake_case)]
            let lpBuffer = self.memory_basic_information.as_mut_ptr() as _;
            #[allow(non_snake_case)]
            let dwLength = size_of::<MemoryBasicInformation>();
            let r#return = unsafe { VirtualQueryEx(hProcess, lpAddress, lpBuffer, dwLength) };
            ensure!(r#return != 0, io::Error::last_os_error());
            ensure!(
                r#return == size_of::<MemoryBasicInformation>(),
                "Invalid buffer size (expected: {}, found: {}).",
                size_of::<MemoryBasicInformation>(),
                r#return,
            );
            let information = unsafe { self.memory_basic_information.assume_init() };
            ensure!(
                information.base_address() == self.address,
                "Invalid base address (expected: {:x}, found: {:x}).",
                self.address,
                information.base_address(),
            );
            Ok(information)
        }
    }

    impl<T> From<super::VirtualQueryBuilder<(T,)>> for VirtualQueryBuilder<'_, ((), T)> {
        fn from(from: super::VirtualQueryBuilder<(T,)>) -> Self {
            Self {
                fields: ((), from.fields.0),
                _phantom: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod read_process_memory {
        use super::ReadProcessMemory;
        use crate::um::processthreadsapi::GetCurrentProcess;
        use anyhow::Result;

        #[test]
        fn with_process_and_base_address_and_buffer() -> Result<()> {
            let process = GetCurrentProcess();
            let address = 9;
            let mut buffer = [9; 9];
            let read_process_memory = ReadProcessMemory::builder()
                .process(&process)
                .base_address(&address)
                .buffer(&mut buffer)
                .build();
            assert_eq!(9, *read_process_memory.base_address);
            assert_eq!(&[9; 9], read_process_memory.buffer);
            assert_eq!(9, read_process_memory.buffer.len());
            Ok(())
        }
    }

    mod virtual_query {
        use super::VirtualQuery;
        use crate::um::processthreadsapi::GetCurrentProcess;
        use anyhow::Result;

        #[test]
        fn virtual_query() -> Result<()> {
            let information = VirtualQuery::builder().address(0).build()()?;
            assert_eq!(information.base_address(), 0);
            Ok(())
        }

        #[test]
        fn extended_virtual_query() -> Result<()> {
            let process = GetCurrentProcess();
            let information = VirtualQuery::builder().process(&process).address(0).build()()?;
            assert_eq!(information.base_address(), 0);
            Ok(())
        }
    }
}
