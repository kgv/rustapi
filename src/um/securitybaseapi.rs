use crate::{shared::ntdef::Handle, um::winnt::Privileges};
use anyhow::{ensure, Error};
use log::warn;
use std::{
    io,
    mem::size_of_val,
    os::windows::io::AsRawHandle,
    ptr::{null, null_mut},
};
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Adjust token privileges.
#[derive(TypedBuilder)]
pub struct AdjustTokenPrivileges<'a> {
    token: &'a Handle,
    #[builder(default)]
    disable_all_privileges: bool,
    #[builder(default, setter(strip_option))]
    new_state: Option<&'a Privileges>,

    #[builder(default, setter(skip))]
    buffer_length: u32,
    #[builder(default, setter(strip_option))]
    previous_state: Option<&'a mut Privileges>,
    #[builder(default, setter(skip))]
    return_length: u32,
}

impl FnOnce<()> for AdjustTokenPrivileges<'_> {
    type Output = Result<(), Error>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::securitybaseapi::AdjustTokenPrivileges;

        #[allow(non_snake_case)]
        let TokenHandle = self.token.as_raw_handle();
        #[allow(non_snake_case)]
        let DisableAllPrivileges = self.disable_all_privileges as _;
        #[allow(non_snake_case)]
        let NewState = self.new_state.map_or(null(), |new_state| &**new_state) as _;
        #[allow(non_snake_case)]
        let BufferLength = self
            .previous_state
            .as_ref()
            .map(|previous_state| size_of_val(*previous_state))
            .unwrap_or_default() as _;
        #[allow(non_snake_case)]
        let PreviousState = self
            .previous_state
            .map_or(null_mut(), |previous_state| &mut **previous_state);
        #[allow(non_snake_case)]
        let ReturnLength = &mut self.return_length;
        let r#return = unsafe {
            AdjustTokenPrivileges(
                TokenHandle,
                DisableAllPrivileges,
                NewState,
                BufferLength,
                PreviousState,
                ReturnLength,
            )
        };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(())
    }
}

// TODO:
// if self.disable_all_privileges.is_some() && self.new_state.is_some() {
//     warn!("The disable_all_privileges and the new_state are set.");
// }
// #[builder(
//      condition = "self.disable_all_privileges.is_some() && self.new_state.is_some()",
//      warn = "The disable_all_privileges and the new_state are set."
// )]
impl<T, W> AdjustTokenPrivilegesBuilder<'_, (T, (bool,), (Option<&Privileges>,), W)> {
    fn validate(&self) -> Result<(), String> {
        let disable_all_privileges = (self.fields.1).0;
        let new_state = (self.fields.2).0;
        if disable_all_privileges && new_state.is_some() {
            warn!("The disable_all_privileges is true and the new_state is some.");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::AdjustTokenPrivileges;
    use crate::um::{
        processthreadsapi::{GetCurrentProcess, OpenProcessToken},
        winnt::Privilege,
    };
    use anyhow::Error;
    use winapi::um::winnt::{SE_PRIVILEGE_ENABLED, SE_SECURITY_NAME, TOKEN_ADJUST_PRIVILEGES};

    #[test]
    fn disable_all_privileges() -> Result<(), Error> {
        let process = GetCurrentProcess();
        let token = OpenProcessToken::builder()
            .process(&process)
            .desired_access(TOKEN_ADJUST_PRIVILEGES)
            .build()()?;
        AdjustTokenPrivileges::builder()
            .token(&token)
            .disable_all_privileges(true)
            .build()()?;
        Ok(())
    }

    #[test]
    fn privileges() -> Result<(), Error> {
        let process = GetCurrentProcess();
        let token = OpenProcessToken::builder()
            .process(&process)
            .desired_access(TOKEN_ADJUST_PRIVILEGES)
            .build()()?;
        let privilege = Privilege::lookup(SE_SECURITY_NAME)?.attribute(SE_PRIVILEGE_ENABLED);
        let privileges = privilege.into();
        AdjustTokenPrivileges::builder()
            .token(&token)
            .new_state(&privileges)
            .build()()?;
        Ok(())
    }
}
