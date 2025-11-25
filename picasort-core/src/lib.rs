// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use std::any::Any;
use struct_introspec_macros::DynamicGetSet;

pub mod error;
pub mod image;
pub mod metadata;
pub mod utils;

#[macro_export]
macro_rules! try_assert {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

pub trait DynamicGetSet {
    fn set_field_by_index(&mut self, index: usize, value: Box<dyn Any>)
    -> Result<(), &'static str>;
    fn set_field_by_name(&mut self, name: &str, value: Box<dyn Any>) -> Result<(), &'static str>;
    fn get_field_names() -> Vec<&'static str>;
    fn get_value_by_field_name(&self, name: &str) -> Option<&dyn std::any::Any>;
}
