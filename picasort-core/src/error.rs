// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use std::{io, string::FromUtf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CoreError {
    /// The GPS data is invalid
    #[error("Invalid GPS data")]
    InvalidGPSData(String),

    /// The EXIF convertion is wrong
    #[error("Invalid EXIF convertion")]
    InvalidEXIFConversion(String),

    /// The EXIF is not found
    #[error("EXIF Tag not found")]
    EXIFTagNotFound(),

    /// Standard IO error
    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    /// Chrono parsing time error
    #[error("Time parse error: {0}")]
    TimeParse(#[from] chrono::ParseError),

    /// Utf8 conversion error
    #[error("UTF-8 conversion error: {0}")]
    Ut8Converion(#[from] FromUtf8Error),
}
