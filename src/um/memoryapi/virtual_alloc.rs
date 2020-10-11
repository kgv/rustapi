use crate::{r#macro::FnOnce, shared::ntdef::Handle};
use anyhow::{ensure, Result};
use std::{io, ptr::NonNull};
use typed_builder::TypedBuilder;

/// Virtual allocation.
#[derive(FnOnce, TypedBuilder)]
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
        unsafe {
            let lpAddress = self.address as _;
            let dwSize = self.size;
            let flAllocationType = self.allocation_type;
            let flProtect = self.protect;
            let r#return = VirtualAlloc(lpAddress, dwSize, flAllocationType, flProtect);
            ensure!(!r#return.is_null(), io::Error::last_os_error());
            Ok(NonNull::new_unchecked(r#return as _))
        }
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

pub mod extended {
    use crate::{r#macro::FnOnce, shared::ntdef::Handle};
    use anyhow::{ensure, Result};
    use std::{io, marker::PhantomData, os::windows::io::AsRawHandle, ptr::NonNull};
    use typed_builder::TypedBuilder;

    /// Virtual allocation extended.
    #[derive(FnOnce, TypedBuilder)]
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
            unsafe {
                let hProcess = self.process.as_raw_handle();
                let lpAddress = self.address as _;
                let dwSize = self.size;
                let flAllocationType = self.allocation_type;
                let flProtect = self.protect;
                let r#return =
                    VirtualAllocEx(hProcess, lpAddress, dwSize, flAllocationType, flProtect);
                ensure!(!r#return.is_null(), io::Error::last_os_error());
                Ok(NonNull::new_unchecked(r#return as _))
            }
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
}
