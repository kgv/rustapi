use rustapi_macro::Handle;

#[derive(Handle)]
pub struct Temp(*mut u8);

#[test]
fn struct_named() {
    // // Should pass
    // StructContainer::from_derive_input(&source::named_struct()).unwrap();

    // // Should fail
    // StructContainer::from_derive_input(&source::tuple_struct()).unwrap_err();
    // StructContainer::from_derive_input(&source::named_field_enum()).unwrap_err();
    // StructContainer::from_derive_input(&source::newtype_enum()).unwrap_err();
}

mod utils {
    /// As strict raw handle.
    pub trait AsStrictRawHandle: FromStrictRawHandle {
        fn as_strict_raw_handle(&self) -> Self::StrictRawHandle;
    }

    /// From strict raw handle.
    pub trait FromStrictRawHandle {
        type StrictRawHandle;

        unsafe fn from_strict_raw_handle(handle: Self::StrictRawHandle) -> Self;
    }

    /// Into strict raw handle.
    pub trait IntoStrictRawHandle: FromStrictRawHandle {
        fn into_strict_raw_handle(self) -> Self::StrictRawHandle;
    }
}
