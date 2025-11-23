// Copyright (c) 2025 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

pub mod descriptor;
pub mod exif;
pub mod gps;

use crate::metadata::{descriptor::Descriptor, gps::GPSData};

#[derive(Debug)]
pub struct TimeData {}

#[derive(Debug)]
pub struct Metadata {
    pub gps_data: Option<GPSData>,
    pub descriptor: Descriptor,
    pub file_path: String,
    // pub time_data: Option<TimeData>,
    // pub image_data: Option<ImageData>,
}
