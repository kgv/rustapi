use crate::{r#macro::FnOnce, um::winnt::Privilege};
use anyhow::{ensure, Error};
use std::{io, mem::MaybeUninit, ptr::null};
use typed_builder::TypedBuilder;
use widestring::WideCString;
use winapi::{
    shared::{minwindef::FALSE, ntdef::LUID},
    um::winnt::LUID_AND_ATTRIBUTES,
};

/// Lookup privilege value.
#[derive(FnOnce, TypedBuilder)]
pub struct LookupPrivilegeValue<'a> {
    #[builder(default)]
    system_name: Option<&'a str>,
    name: &'a str,
    #[builder(default = MaybeUninit::zeroed(), setter(skip))]
    luid: MaybeUninit<LUID>,
}

impl FnOnce<()> for LookupPrivilegeValue<'_> {
    type Output = Result<Privilege, Error>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::winbase::LookupPrivilegeValueW;

        let system_name = self.system_name.map(WideCString::from_str).transpose()?;
        let name = WideCString::from_str(self.name)?;

        #[allow(non_snake_case)]
        unsafe {
            let lpSystemName = system_name.map_or(null(), |v| v.as_ptr());
            let lpName = name.as_ptr();
            let lpLuid = self.luid.as_mut_ptr();
            let r#return = LookupPrivilegeValueW(lpSystemName, lpName, lpLuid);
            ensure!(r#return != FALSE, io::Error::last_os_error());
            Ok(Privilege::from(LUID_AND_ATTRIBUTES {
                Luid: self.luid.assume_init(),
                Attributes: 0,
            }))
        }
    }
}
