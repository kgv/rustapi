pub(crate) use self::get_mapped_file_name::{GetMappedFileName, GetMappedFileNameBuilder};

pub fn get_mapped_file_name<'a>() -> GetMappedFileNameBuilder<'a, ((), ())> {
    GetMappedFileName::builder()
}

mod get_mapped_file_name;
