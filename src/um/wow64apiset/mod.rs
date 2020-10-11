pub(crate) use self::is_wow64_process::{IsWow64Process, IsWow64ProcessBuilder};

pub fn is_wow64_process<'a>() -> IsWow64ProcessBuilder<'a, ((),)> {
    IsWow64Process::builder()
}

mod is_wow64_process;
