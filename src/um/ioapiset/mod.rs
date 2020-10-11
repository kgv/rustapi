pub(crate) use self::device_io_control::{DeviceIoControl, DeviceIoControlBuilder};

pub fn device_io_control<'a>() -> DeviceIoControlBuilder<'a, ((), (), (), (), ())> {
    DeviceIoControl::builder()
}

mod device_io_control;
