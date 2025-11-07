// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

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
