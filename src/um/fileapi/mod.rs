pub(crate) use self::{
    create_file::{CreateFile, CreateFileBuilder},
    get_logical_drive_strings::{GetLogicalDriveStrings, GetLogicalDriveStringsBuilder},
    query_dos_device::{QueryDosDevice, QueryDosDeviceBuilder},
};

pub fn create_file<'a>() -> CreateFileBuilder<'a, ((), (), (), (), (), (), ())> {
    CreateFile::builder()
}

pub fn get_logical_drive_strings() -> GetLogicalDriveStringsBuilder<((),)> {
    GetLogicalDriveStrings::builder()
}

pub fn query_dos_device<'a>() -> QueryDosDeviceBuilder<'a, ((), ())> {
    QueryDosDevice::builder()
}

mod create_file;
mod get_logical_drive_strings;
mod query_dos_device;
