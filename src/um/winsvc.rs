use bitflags::bitflags;
use derive_more::{From, Into};
use std::fmt::{self, Debug, Formatter};
use winapi::{
    shared::minwindef::DWORD,
    um::{
        winnt::{
            SERVICE_FILE_SYSTEM_DRIVER, SERVICE_INTERACTIVE_PROCESS, SERVICE_KERNEL_DRIVER,
            SERVICE_USER_OWN_PROCESS, SERVICE_USER_SHARE_PROCESS, SERVICE_WIN32_OWN_PROCESS,
            SERVICE_WIN32_SHARE_PROCESS,
        },
        winsvc::{
            SERVICE_ACCEPT_HARDWAREPROFILECHANGE, SERVICE_ACCEPT_NETBINDCHANGE,
            SERVICE_ACCEPT_PARAMCHANGE, SERVICE_ACCEPT_PAUSE_CONTINUE, SERVICE_ACCEPT_POWEREVENT,
            SERVICE_ACCEPT_PRESHUTDOWN, SERVICE_ACCEPT_SESSIONCHANGE, SERVICE_ACCEPT_SHUTDOWN,
            SERVICE_ACCEPT_STOP, SERVICE_ACCEPT_TIMECHANGE, SERVICE_ACCEPT_TRIGGEREVENT,
            SERVICE_CONTINUE_PENDING, SERVICE_PAUSED, SERVICE_PAUSE_PENDING, SERVICE_RUNNING,
            SERVICE_START_PENDING, SERVICE_STATUS, SERVICE_STOPPED, SERVICE_STOP_PENDING,
        },
    },
};

const SERVICE_ACCEPT_USERMODEREBOOT: DWORD = 0x00000080;

/// Status.
#[derive(From, Into)]
#[repr(transparent)]
pub struct Status(SERVICE_STATUS);

impl Status {
    #[inline]
    pub fn service_type(&self) -> Type {
        Type::from(self.0.dwServiceType)
    }

    #[inline]
    pub fn current_state(&self) -> CurrentState {
        CurrentState::from(self.0.dwCurrentState)
    }

    #[inline]
    pub fn controls_accepted(&self) -> ControlAccept {
        ControlAccept::from_bits_truncate(self.0.dwControlsAccepted)
    }

    #[inline]
    pub fn win32_exit_code(&self) -> u32 {
        self.0.dwWin32ExitCode
    }

    #[inline]
    pub fn service_specific_exit_code(&self) -> u32 {
        self.0.dwServiceSpecificExitCode
    }

    #[inline]
    pub fn check_point(&self) -> u32 {
        self.0.dwCheckPoint
    }

    #[inline]
    pub fn wait_hint(&self) -> u32 {
        self.0.dwWaitHint
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Status")
            .field("service_type", &self.service_type())
            .field("current_state", &self.current_state())
            .field("controls_accepted", &self.controls_accepted())
            .field("win32_exit_code", &self.win32_exit_code())
            .field(
                "service_specific_exit_code",
                &self.service_specific_exit_code(),
            )
            .field("check_point", &self.check_point())
            .field("wait_hint", &self.wait_hint())
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    FileSystemDriver,
    KernelDriver,
    Win32OwnProcess,
    Win32ShareProcess,
    UserOwnProcess,
    UserShareProcess,
    InteractiveProcess,
}

impl From<Type> for u32 {
    fn from(from: Type) -> Self {
        match from {
            Type::FileSystemDriver => SERVICE_FILE_SYSTEM_DRIVER,
            Type::KernelDriver => SERVICE_KERNEL_DRIVER,
            Type::Win32OwnProcess => SERVICE_WIN32_OWN_PROCESS,
            Type::Win32ShareProcess => SERVICE_WIN32_SHARE_PROCESS,
            Type::UserOwnProcess => SERVICE_USER_OWN_PROCESS,
            Type::UserShareProcess => SERVICE_USER_SHARE_PROCESS,
            Type::InteractiveProcess => SERVICE_INTERACTIVE_PROCESS,
        }
    }
}

impl From<u32> for Type {
    fn from(from: u32) -> Self {
        match from {
            SERVICE_FILE_SYSTEM_DRIVER => Type::FileSystemDriver,
            SERVICE_KERNEL_DRIVER => Type::KernelDriver,
            SERVICE_WIN32_OWN_PROCESS => Type::Win32OwnProcess,
            SERVICE_WIN32_SHARE_PROCESS => Type::Win32ShareProcess,
            SERVICE_USER_OWN_PROCESS => Type::UserOwnProcess,
            SERVICE_USER_SHARE_PROCESS => Type::UserShareProcess,
            SERVICE_INTERACTIVE_PROCESS => Type::InteractiveProcess,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurrentState {
    ContinuePending,
    PausePending,
    Paused,
    Running,
    StartPending,
    StopPending,
    Stopped,
}

impl From<CurrentState> for u32 {
    fn from(from: CurrentState) -> Self {
        match from {
            CurrentState::ContinuePending => SERVICE_CONTINUE_PENDING,
            CurrentState::PausePending => SERVICE_PAUSE_PENDING,
            CurrentState::Paused => SERVICE_PAUSED,
            CurrentState::Running => SERVICE_RUNNING,
            CurrentState::StartPending => SERVICE_START_PENDING,
            CurrentState::StopPending => SERVICE_STOP_PENDING,
            CurrentState::Stopped => SERVICE_STOPPED,
        }
    }
}

impl From<u32> for CurrentState {
    fn from(from: u32) -> Self {
        match from {
            SERVICE_CONTINUE_PENDING => CurrentState::ContinuePending,
            SERVICE_PAUSE_PENDING => CurrentState::PausePending,
            SERVICE_PAUSED => CurrentState::Paused,
            SERVICE_RUNNING => CurrentState::Running,
            SERVICE_START_PENDING => CurrentState::StartPending,
            SERVICE_STOP_PENDING => CurrentState::StopPending,
            SERVICE_STOPPED => CurrentState::Stopped,
            _ => unimplemented!(),
        }
    }
}

bitflags! {
    /// Control accept.
    pub struct ControlAccept: u32 {
        const NETBIND_CHANGE = SERVICE_ACCEPT_NETBINDCHANGE;
        const PARAM_CHANGE = SERVICE_ACCEPT_PARAMCHANGE;
        const PAUSE_CONTINUE = SERVICE_ACCEPT_PAUSE_CONTINUE;
        const PRESHUTDOWN = SERVICE_ACCEPT_PRESHUTDOWN;
        const SHUTDOWN = SERVICE_ACCEPT_SHUTDOWN;
        const STOP = SERVICE_ACCEPT_STOP;
        const HARDWARE_PROFILE_CHANGE = SERVICE_ACCEPT_HARDWAREPROFILECHANGE;
        const POWER_EVENT = SERVICE_ACCEPT_POWEREVENT;
        const SESSION_CHANGE = SERVICE_ACCEPT_SESSIONCHANGE;
        const TIME_CHANGE = SERVICE_ACCEPT_TIMECHANGE;
        const TRIGGER_EVENT = SERVICE_ACCEPT_TRIGGEREVENT;
        const USERMODE_REBOOT = SERVICE_ACCEPT_USERMODEREBOOT;
    }
}
