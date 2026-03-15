// Copyright (c) 2026 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::metadata::exif::{
    extract_orientation, extract_string, extract_unsigned_int16, extract_unsigned_int32,
    extract_utc_datetime, ExifAssignable, ExtractionSet, TagContext,
};
use crate::DynamicGetSet;
use chrono::{DateTime, Utc};

use little_exif::exif_tag::ExifTag;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Orientation {
    Normal,
    FlippedHorizontally,
    Rotated180Deg,
    FlippedVertically,
    Rotated90DegCCWFlippedVertically,
    Rotated90DegCW,
    Rotated90DegCCWPFlippedHorizontally,
    Rotated90DegCCW,
    Unknown,
}
