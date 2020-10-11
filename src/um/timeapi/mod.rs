pub(crate) use self::time_get_time::TimeGetTime;

pub fn time_get_time() -> u32 {
    TimeGetTime()
}

mod time_get_time;
