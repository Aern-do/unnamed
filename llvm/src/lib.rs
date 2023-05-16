#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::ffi::CString;
pub mod context;
pub mod types;
pub mod module;
pub mod values;
pub mod builder;

pub fn to_c_str(name: &str) -> CString {
    CString::new(name).expect("CString::new failed")
}