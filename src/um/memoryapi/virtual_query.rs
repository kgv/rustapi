use crate::{r#macro::FnOnce, shared::ntdef::Handle, um::winnt::MemoryBasicInformation};
use anyhow::{ensure, Result};
use std::{
    io,
    mem::{size_of, MaybeUninit},
};
use typed_builder::TypedBuilder;

/// Virtual query.
#[derive(FnOnce, TypedBuilder)]
pub struct VirtualQuery {
    address: usize,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    memory_basic_information: MaybeUninit<MemoryBasicInformation>,
}

impl FnOnce<()> for VirtualQuery {
    type Output = Result<MemoryBasicInformation>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::memoryapi::VirtualQuery;

        #[allow(non_snake_case)]
        unsafe {
            let lpAddress = self.address as _;
            let lpBuffer = self.memory_basic_information.as_mut_ptr() as _;
            let dwLength = size_of::<MemoryBasicInformation>();
            let r#return = VirtualQuery(lpAddress, lpBuffer, dwLength);
            ensure!(r#return != 0, io::Error::last_os_error());
            ensure!(
                r#return == size_of::<MemoryBasicInformation>(),
                "Invalid buffer size (expected: {}, found: {}).",
                size_of::<MemoryBasicInformation>(),
                r#return,
            );
            let information = self.memory_basic_information.assume_init();
            ensure!(
                information.base_address() == self.address,
                "Invalid base address (expected: {:x}, found: {:x}).",
                self.address,
                information.base_address(),
            );
            Ok(information)
        }
    }
}

impl<T> VirtualQueryBuilder<(T,)> {
    pub fn process(self, process: &Handle) -> extended::VirtualQueryBuilder<'_, ((&Handle,), T)> {
        extended::VirtualQueryBuilder::from(self).process(process)
    }
}

pub mod extended {
    use crate::{r#macro::FnOnce, shared::ntdef::Handle, um::winnt::MemoryBasicInformation};
    use anyhow::{ensure, Result};
    use std::{
        io,
        marker::PhantomData,
        mem::{size_of, MaybeUninit},
        os::windows::io::AsRawHandle,
    };
    use typed_builder::TypedBuilder;

    /// Virtual query extended.
    #[derive(FnOnce, TypedBuilder)]
    pub struct VirtualQuery<'a> {
        process: &'a Handle,
        address: usize,
        #[builder(default = MaybeUninit::zeroed(), setter(skip))]
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
