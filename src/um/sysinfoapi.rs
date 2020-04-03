use derive_more::{Deref, DerefMut, From, Into};
use lazy_static::lazy_static;
use std::{
    fmt::{self, Debug, Formatter},
    mem::MaybeUninit,
    ops::RangeInclusive,
};
use typed_builder::TypedBuilder;
use winapi::um::{
    sysinfoapi::SYSTEM_INFO,
    winnt::{
        PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM, PROCESSOR_ARCHITECTURE_ARM64,
        PROCESSOR_ARCHITECTURE_IA64, PROCESSOR_ARCHITECTURE_INTEL, PROCESSOR_ARCHITECTURE_UNKNOWN,
    },
};

// lazy_static! {
//     pub static ref ALLOCATION_GRANULARITY: u32 = SystemInformation::new().allocation_granularity();
//     pub static ref PAGE_SIZE: u32 = SystemInformation::new().page_size();
//     pub static ref PROCESSOR_ARCHITECTURE: ProcessorArchitecture =
//         SystemInformation::new().processor_architecture();
//     pub static ref PAGE_SHIFT: usize = {
//         assert!(PAGE_SIZE.is_power_of_two());
//         PAGE_SIZE.trailing_zeros() as _
//     };
// }

/// Get native system information.
#[derive(TypedBuilder)]
pub struct GetNativeSystemInfo {
    #[builder(default = MaybeUninit::uninit(), setter(skip))]
    system_information: MaybeUninit<SystemInformation>,
}

impl FnOnce<()> for GetNativeSystemInfo {
    type Output = SystemInformation;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::sysinfoapi::GetNativeSystemInfo;

        #[allow(non_snake_case)]
        let lpSystemInfo = self.system_information.as_mut_ptr() as _;
        unsafe { GetNativeSystemInfo(lpSystemInfo) };
        unsafe { self.system_information.assume_init() }
    }
}

/// Get system information.
#[derive(TypedBuilder)]
pub struct GetSystemInfo {
    #[builder(default = MaybeUninit::uninit(), setter(skip))]
    system_information: MaybeUninit<SystemInformation>,
}

impl Default for GetSystemInfo {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl FnOnce<()> for GetSystemInfo {
    type Output = SystemInformation;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::sysinfoapi::GetSystemInfo;

        #[allow(non_snake_case)]
        let lpSystemInfo = self.system_information.as_mut_ptr() as _;
        unsafe { GetSystemInfo(lpSystemInfo) };
        unsafe { self.system_information.assume_init() }
    }
}

/// Get tick count.
pub struct GetTickCount;

impl FnOnce<()> for GetTickCount {
    type Output = u32;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::sysinfoapi::GetTickCount;

        unsafe { GetTickCount() }
    }
}

/// System information.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct SystemInformation(SYSTEM_INFO);

impl SystemInformation {
    pub fn new() -> Self {
        GetSystemInfo::default()()
    }

    #[inline]
    pub fn allocation_granularity(&self) -> u32 {
        self.0.dwAllocationGranularity
    }

    #[inline]
    pub fn application_addresses(&self) -> RangeInclusive<usize> {
        self.0.lpMinimumApplicationAddress as _..=self.0.lpMaximumApplicationAddress as _
    }

    #[inline]
    pub fn page_size(&self) -> u32 {
        self.0.dwPageSize
    }

    #[inline]
    pub fn processor_architecture(&self) -> ProcessorArchitecture {
        unsafe { self.0.u.s().wProcessorArchitecture.into() }
    }
}

impl Debug for SystemInformation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("SystemInformation")
            .field("processor_architecture", &self.processor_architecture())
            .field("page_size", &self.page_size())
            .field(
                "application_addresses",
                &format_args!(
                    "{:#x}..={:#x}",
                    self.application_addresses().start(),
                    self.application_addresses().end()
                ),
            )
            .field("allocation_granularity", &self.allocation_granularity())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessorArchitecture {
    Intel,
    Amd64,
    Arm,
    Arm64,
    Ia64,
    Unknown,
}

impl From<u16> for ProcessorArchitecture {
    fn from(from: u16) -> ProcessorArchitecture {
        match from {
            PROCESSOR_ARCHITECTURE_INTEL => Self::Intel,
            PROCESSOR_ARCHITECTURE_AMD64 => Self::Amd64,
            PROCESSOR_ARCHITECTURE_ARM => Self::Arm,
            PROCESSOR_ARCHITECTURE_ARM64 => Self::Arm64,
            PROCESSOR_ARCHITECTURE_IA64 => Self::Ia64,
            PROCESSOR_ARCHITECTURE_UNKNOWN | _ => Self::Unknown,
        }
    }
}
