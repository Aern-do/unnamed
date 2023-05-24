#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::ffi::CString;
pub mod attribute;
pub mod builder;
pub mod context;
pub mod extra;
pub mod module;
pub mod types;
pub mod values;

pub struct Assert<const COND: bool>;

pub trait True {}
pub trait False {}

impl True for Assert<true> {}
impl False for Assert<false> {}

pub fn to_c_str(str: &str) -> CString {
    CString::new(str).unwrap()
}
