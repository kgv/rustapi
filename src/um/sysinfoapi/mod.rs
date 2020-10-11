pub(crate) use self::{
    get_native_system_info::GetNativeSystemInfo, get_system_info::GetSystemInfo,
    get_tick_count::GetTickCount,
};
use derive_more::{Deref, DerefMut, From, Into};
use std::{
    fmt::{self, Debug, Formatter},
    ops::RangeInclusive,
};
use winapi::um::{
    sysinfoapi::SYSTEM_INFO,
    winnt::{
        PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM, PROCESSOR_ARCHITECTURE_ARM64,
        PROCESSOR_ARCHITECTURE_IA64, PROCESSOR_ARCHITECTURE_INTEL, PROCESSOR_ARCHITECTURE_UNKNOWN,
    },
};

pub fn get_native_system_information() -> SystemInformation {
    GetNativeSystemInfo::builder().build()()
}

pub fn get_system_information() -> SystemInformation {
    GetSystemInfo::builder().build()()
}

pub fn get_tick_count() -> u32 {
    GetTickCount()
}

mod get_native_system_info;
mod get_system_info;
mod get_tick_count;

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
