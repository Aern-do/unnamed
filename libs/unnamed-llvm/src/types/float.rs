use crate::{impl_as_type_ref, impl_type_downcast};

use super::BaseType;

#[derive(Debug, Clone, Copy)]
pub struct HalfType<'ctx>(BaseType<'ctx>);

#[derive(Debug, Clone, Copy)]
pub struct FloatType<'ctx>(BaseType<'ctx>);

#[derive(Debug, Clone, Copy)]
pub struct DoubleType<'ctx>(BaseType<'ctx>);

impl_as_type_ref!(HalfType);
impl_type_downcast!(Half);

impl_as_type_ref!(FloatType);
impl_type_downcast!(Float);

impl_as_type_ref!(DoubleType);
impl_type_downcast!(Double);
