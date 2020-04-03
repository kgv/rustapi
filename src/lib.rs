//! - unsafe минимальный блок, все safe пояснения не к конкретному блоку, а к
//!   функции.
//! - #[allow(non_snake_case)] каждый идентификатор получает свой атрибут.
//! - входные параметры - все rust
//! - derive_more::{Clone, (Copy,) Deref, DerefMut, From, Into};
//! - сравнение с тем типом, который возвращает функция (FALSE а не 0)
//!
//! - Первая строка описания (это не заголовок, а именно строка описания)
//!   заканчивается точкой.

#![feature(bool_to_option)]
#![feature(fn_traits)]
#![feature(specialization)]
#![feature(split_inclusive)]
#![feature(unboxed_closures)]

pub mod shared;
pub mod um;

pub mod utils;
pub mod wrap;
