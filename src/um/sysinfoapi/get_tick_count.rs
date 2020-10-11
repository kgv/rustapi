/// Get tick count.
pub struct GetTickCount;

impl FnOnce<()> for GetTickCount {
    type Output = u32;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        use winapi::um::sysinfoapi::GetTickCount;

        unsafe { GetTickCount() }
    }
}