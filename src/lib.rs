//! - unsafe - логический блок, длящийся до какого-либо логического завершения.
//! - #[allow(non_snake_case)] атрибут привязан к unsafe.
//! - входные параметры - все rust
//! - derive_more::{Clone, (Copy,) Deref, DerefMut, From, Into};
//! - сравнение с тем типом, который возвращает функция (FALSE а не 0)
//!
//! - Первая строка описания (это не заголовок, а именно строка описания) заканчивается точкой.
//!
//! `function_without_any_arguments()`
//! `function_without_input_arguments()()`
//! `function_with_input_arguments().arg0()...argn()()`
//!
//! [NTSTATUS to io::Error](https://github.com/rust-lang/rust/pull/41684)

// Close handle:
//
// Access token
// Communications device
// Console input
// Console screen buffer
// Event
// File
// File mapping
// Job
// Mailslot
// Mutex
// Named pipe
// Process
// Semaphore
// Socket
// Thread

// #![feature(generic_associated_types)]
#![allow(incomplete_features)]
#![feature(associated_type_defaults)]
#![feature(bool_to_option)]
#![feature(fn_traits)]
#![feature(maybe_uninit_ref)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(min_const_generics)]
#![feature(specialization)]
#![feature(split_inclusive)]
#![feature(trait_alias)]
#![feature(unboxed_closures)]

pub use rustapi_macro as r#macro;

pub mod shared;
pub mod um;
pub mod utils;
pub mod wrap;

#[test]
fn test() {
    use um::libloaderapi::disable_thread_library_calls;
    use winapi::shared::minwindef::HMODULE;

    // disable_thread_library_calls().handle(0 as HMODULE).build()().unwrap();
    let t = disable_thread_library_calls().handle(0 as HMODULE)();
    println!("t: {:?}", t);
}
