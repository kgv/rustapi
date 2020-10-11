use crate::{
    shared::ntdef::Luid,
    um::{
        processthreadsapi::{GetCurrentProcess, OpenProcessToken},
        securitybaseapi::AdjustTokenPrivileges,
        winbase::LookupPrivilegeValue,
    },
};
use anyhow::Result;
use derive_more::{Deref, DerefMut, Display, From, Into};
use std::fmt::{self, Debug, Display, Formatter};
use winapi::um::winnt::{
    LUID_AND_ATTRIBUTES, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_FREE, MEM_IMAGE, MEM_MAPPED,
    MEM_PRIVATE, MEM_RESERVE, PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
    PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_NOCACHE, PAGE_READONLY, PAGE_READWRITE,
    PAGE_WRITECOMBINE, PAGE_WRITECOPY, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES,
    TOKEN_PRIVILEGES,
};

/// Memory basic information.
#[derive(Clone, Deref, DerefMut, From, Into)]
#[repr(transparent)]
pub struct MemoryBasicInformation(MEMORY_BASIC_INFORMATION);

// - MEM_FREE: элементы AllocationBase, AllocationProtect, Protect и Type
//   содержат неопределенные значения,
// - MEM_RESERVE: неопределенное значение содержит элемент Protect.
impl MemoryBasicInformation {
    #[inline]
    pub fn state(&self) -> State {
        State::from(self.0.State)
    }

    #[inline]
    pub fn r#type(&self) -> Option<Type> {
        match self.state() {
            State::Free => None,
            _ => Some(Type::from(self.0.Type)),
        }
    }

    #[inline]
    pub fn base_address(&self) -> usize {
        self.0.BaseAddress as _
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.0.RegionSize
    }

    #[inline]
    pub fn protect(&self) -> Option<Protect> {
        match self.state() {
            State::Free | State::Reserve => None,
            _ => Some(Protect::from(self.0.Protect)),
        }
    }

    #[inline]
    pub fn allocation_base_address(&self) -> Option<usize> {
        match self.state() {
            State::Free => None,
            _ => Some(self.0.AllocationBase as _),
        }
    }

    #[inline]
    pub fn allocation_protect(&self) -> Option<Protect> {
        match self.state() {
            State::Free => None,
            _ => Some(Protect::from(self.0.AllocationProtect)),
        }
    }
}

impl Debug for MemoryBasicInformation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("MemoryBasicInformation")
            .field("state", &self.state())
            .field("type", &self.r#type())
            .field("base_address", &self.base_address())
            .field("size", &self.size())
            .field("protect", &self.protect())
            .field("allocation_base_address", &self.allocation_base_address())
            .field("allocation_protect", &self.allocation_protect())
            .finish()
    }
}

/// Privilege.
#[derive(Clone, Copy, Deref, Display, From, Into)]
#[display(fmt = "luid: {}", "self.luid()")]
#[repr(transparent)]
pub struct Privilege(LUID_AND_ATTRIBUTES);

impl Privilege {
    /// Enable privilege for current process.
    #[inline(never)]
    pub fn enable(name: &str) -> Result<()> {
        let process = GetCurrentProcess();
        let token = OpenProcessToken::builder()
            .process_handle(&process)
            .desired_access(TOKEN_ADJUST_PRIVILEGES)
            .build()()?;
        let privilege = Privilege::lookup(name)?.attribute(SE_PRIVILEGE_ENABLED);
        let privileges = privilege.into();
        AdjustTokenPrivileges::builder()
            .token(&token)
            .new_state(&privileges)
            .build()()?;
        Ok(())
    }

    /// Lookup privilege on the local system.
    pub fn lookup(name: &str) -> Result<Self> {
        LookupPrivilegeValue::builder().name(name).build()()
    }

    #[inline]
    pub fn luid(&self) -> Luid {
        self.0.Luid.into()
    }

    #[inline]
    pub fn attributes(&self) -> u32 {
        self.0.Attributes
    }

    pub fn attribute(mut self, attribute: u32) -> Self {
        self.0.Attributes |= attribute;
        self
    }
}

/// Privileges.
#[derive(Clone, Copy, DerefMut, Deref)]
#[repr(transparent)]
pub struct Privileges(TOKEN_PRIVILEGES);

impl Privileges {
    // fn new<I: ExactSizeIterator<Item = Privilege>>(iter: I) -> Self {
    //     Self(TOKEN_PRIVILEGES {
    //         PrivilegeCount: iter.len() as _,
    //         Privileges: [iter[0].0],
    //     })
    // }

    #[inline]
    pub fn count(&self) -> usize {
        self.0.PrivilegeCount as _
    }
}

impl From<Privilege> for Privileges {
    fn from(from: Privilege) -> Self {
        Self(TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [from.0],
        })
    }
}

/// Protect.
#[derive(Clone, Copy, Debug, Default)]
pub struct Protect {
    pub access: Access,
    pub flags: Flags,
}

impl Protect {
    /// Enable execute access.
    pub fn execute(mut self) -> Self {
        self.access.execute = true;
        self
    }

    /// Enable read access.
    pub fn read(mut self) -> Self {
        self.access.read = true;
        self
    }

    /// Enable read/write access.
    pub fn write(mut self) -> Self {
        self.access.read = true;
        self.access.write = true;
        self
    }

    /// Enable read/copy-on-write access.
    pub fn copy(mut self) -> Self {
        self.access.read = true;
        self.access.write = true;
        self.access.copy = true;
        self
    }

    /// Enable guard flag.
    pub fn guard(mut self) -> Self {
        self.flags.guard = true;
        self
    }

    /// Enable no-cache flag.
    pub fn no_cache(mut self) -> Self {
        self.flags.no_cache = true;
        self
    }

    /// Enable write-combine flag.
    pub fn write_combine(mut self) -> Self {
        self.flags.write_combine = true;
        self
    }
}

impl Display for Protect {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let execute = if self.access.execute { 'e' } else { '-' };
        let read = if self.access.execute { 'r' } else { '-' };
        let write = if self.access.execute { 'w' } else { '-' };
        let copy = if self.access.execute { 'c' } else { '-' };
        let guard = if self.access.execute { 'G' } else { '-' };
        let no_cache = if self.access.execute { 'C' } else { '-' };
        let write_combine = if self.access.execute { 'W' } else { '-' };
        f.debug_tuple("Protect")
            .field(&format_args!(
                "[{}{}{}{}{}{}{}]",
                execute, read, write, copy, guard, no_cache, write_combine
            ))
            .finish()
    }
}

impl From<Protect> for u32 {
    fn from(from: Protect) -> u32 {
        let execute = from.access.execute;
        let read = from.access.read;
        let write = from.access.write;
        let copy = from.access.copy;

        let mut to = if !execute && !read && !write && !copy {
            PAGE_NOACCESS
        } else if !execute && read && !write && !copy {
            PAGE_READONLY
        } else if !execute && read && write && !copy {
            PAGE_READWRITE
        } else if !execute && read && write && copy {
            PAGE_WRITECOPY
        } else if execute && !read && !write && !copy {
            PAGE_EXECUTE
        } else if execute && read && !write && !copy {
            PAGE_EXECUTE_READ
        } else if execute && read && write && !copy {
            PAGE_EXECUTE_READWRITE
        } else if execute && read && write && copy {
            PAGE_EXECUTE_WRITECOPY
        } else {
            unimplemented!();
        };
        let guard = from.flags.guard;
        let no_cache = from.flags.no_cache;
        let write_combine = from.flags.write_combine;
        if guard {
            to |= PAGE_GUARD;
        }
        if no_cache {
            to |= PAGE_NOCACHE;
        }
        if write_combine {
            to |= PAGE_WRITECOMBINE;
        }
        to
    }
}

impl From<u32> for Protect {
    fn from(from: u32) -> Self {
        let access = match from & 0x00FF {
            PAGE_NOACCESS => Access::default(),
            PAGE_READONLY => Access {
                read: true,
                ..Default::default()
            },
            PAGE_READWRITE => Access {
                read: true,
                write: true,
                ..Default::default()
            },
            PAGE_WRITECOPY => Access {
                read: true,
                write: true,
                copy: true,
                ..Default::default()
            },
            PAGE_EXECUTE => Access {
                execute: true,
                ..Default::default()
            },
            PAGE_EXECUTE_READ => Access {
                execute: true,
                read: true,
                ..Default::default()
            },
            PAGE_EXECUTE_READWRITE => Access {
                execute: true,
                read: true,
                write: true,
                ..Default::default()
            },
            PAGE_EXECUTE_WRITECOPY => Access {
                execute: true,
                read: true,
                write: true,
                copy: true,
            },
            _ => unimplemented!(),
        };
        let mut flags = Flags::default();
        if from & PAGE_GUARD != 0 {
            flags.guard = true;
        }
        if from & PAGE_NOCACHE != 0 {
            flags.no_cache = true;
        }
        if from & PAGE_WRITECOMBINE != 0 {
            flags.write_combine = true;
        }
        Protect { access, flags }
    }
}

/// Access.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Access {
    pub execute: bool,
    pub read: bool,
    pub write: bool,
    pub copy: bool,
}

/// Flags.
#[derive(Clone, Copy, Debug, Default)]
pub struct Flags {
    pub guard: bool,
    pub no_cache: bool,
    pub write_combine: bool,
}

/// State.
#[derive(Clone, Copy, Debug)]
pub enum State {
    Free,
    Reserve,
    Commit,
}

impl From<State> for u32 {
    fn from(from: State) -> u32 {
        match from {
            State::Free => MEM_FREE,
            State::Reserve => MEM_RESERVE,
            State::Commit => MEM_COMMIT,
        }
    }
}

impl From<u32> for State {
    fn from(from: u32) -> Self {
        match from {
            MEM_FREE => State::Free,
            MEM_RESERVE => State::Reserve,
            MEM_COMMIT => State::Commit,
            _ => unimplemented!(),
        }
    }
}

/// Type.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    Image,
    Mapped,
    Private,
}

impl From<Type> for u32 {
    fn from(from: Type) -> u32 {
        match from {
            Type::Image => MEM_IMAGE,
            Type::Mapped => MEM_MAPPED,
            Type::Private => MEM_PRIVATE,
        }
    }
}

impl From<u32> for Type {
    fn from(from: u32) -> Self {
        match from {
            MEM_IMAGE => Type::Image,
            MEM_MAPPED => Type::Mapped,
            MEM_PRIVATE => Type::Private,
            _ => unimplemented!(),
        }
    }
}
