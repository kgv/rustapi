use anyhow::{ensure, Result};
use std::io;
use typed_builder::TypedBuilder;
use winapi::shared::minwindef::FALSE;

/// Query performance counter.
#[derive(TypedBuilder)]
pub struct QueryPerformanceCounter {
    #[builder(default, setter(skip))]
    performance_count: i64,
}

impl Default for QueryPerformanceCounter {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl FnOnce<()> for QueryPerformanceCounter {
    type Output = Result<i64>;

    extern "rust-call" fn call_once(mut self, _args: ()) -> Self::Output {
        use winapi::um::profileapi::QueryPerformanceCounter;

        #[allow(non_snake_case)]
        let lpPerformanceCount = &mut self.performance_count as *mut _ as _;
        let r#return = unsafe { QueryPerformanceCounter(lpPerformanceCount) };
        ensure!(r#return != FALSE, io::Error::last_os_error());
        Ok(self.performance_count)
    }
}
