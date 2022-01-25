#![allow(non_camel_case_types, non_upper_case_globals)]

use std::marker::PhantomData;

#[doc(hidden)]
pub struct MustBeChar<const char: char>;

#[doc(hidden)]
pub struct MustBePosInt<const u128: u128>;

#[doc(hidden)]
pub struct MustBeNegInt<const i128: i128>;

#[doc(hidden)]
pub struct MustBeU8<const u8: u8>;

#[doc(hidden)]
pub struct MustBeU16<const u16: u16>;

#[doc(hidden)]
pub struct MustBeU32<const u32: u32>;

#[doc(hidden)]
pub struct MustBeU64<const u64: u64>;

#[doc(hidden)]
pub struct MustBeU128<const u128: u128>;

#[doc(hidden)]
pub struct MustBeI8<const i8: i8>;

#[doc(hidden)]
pub struct MustBeI16<const i16: i16>;

#[doc(hidden)]
pub struct MustBeI32<const i32: i32>;

#[doc(hidden)]
pub struct MustBeI64<const i64: i64>;

#[doc(hidden)]
pub struct MustBeI128<const i128: i128>;

#[doc(hidden)]
pub struct MustBeBool<const bool: bool>;

#[doc(hidden)]
pub struct MustBeStr<str>(PhantomData<str>);
