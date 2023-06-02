#![allow(incomplete_features)]
#![feature(generic_const_exprs, type_changing_struct_update)]

use std::ffi::CString;

use context::Context;

pub mod attribute;
pub mod builder;
pub mod context;
pub mod error;
pub mod extra;
pub mod module;
pub mod pass_manager;
pub mod target;
pub mod types;
pub mod values;

pub struct Assert<const COND: bool>;

pub trait True {}
pub trait False {}

impl True for Assert<true> {}
impl False for Assert<false> {}

pub trait AsRaw {
    type Raw;
    fn as_raw(&self) -> Self::Raw;
}

pub fn to_c_str(str: &str) -> CString {
    CString::new(str).unwrap()
}

pub trait GetContext {
    fn get_context(&self) -> &Context;
}

#[macro_export]
macro_rules! impl_get_context {
    ($name: ident$(<$lt: lifetime>)?.$field: tt) => {
        impl$(<$lt>)? $crate::GetContext for $name$(<$lt>)? {
            fn get_context(&self) -> &$crate::context::Context {
                self.$field.get_context()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_as_raw {
    ($name: ident$(<$lt: lifetime>)?.$field: tt -> $raw: ident) => {
        impl$(<$lt>)? $crate::AsRaw for $name$(<$lt>)? {
            type Raw = $raw;

            fn as_raw(&self) -> Self::Raw {
                self.$field
            }
        }
    };

    (@downcast $name: ident$(<$lt: lifetime>)?.$field: tt -> $raw: ident) => {
        impl$(<$lt>)? $crate::AsRaw for $name$(<$lt>)? {
            type Raw = $raw;

            fn as_raw(&self) -> Self::Raw {
                self.$field.as_raw()
            }
        }
    };
}
