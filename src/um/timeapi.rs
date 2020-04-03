/// Get tick count.
pub struct TimeGetTime;

impl FnOnce<()> for TimeGetTime {
    type Output = u32;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::timeapi::timeGetTime;

        unsafe { timeGetTime() }
    }
}
