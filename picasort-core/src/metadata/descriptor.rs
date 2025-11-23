// Copyright (c) 2025 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::metadata::exif::{ExifExtractable, get_tag_value};
use chrono::{DateTime, Utc};

use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;

use crate::error::CoreError;

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertival,
    Other,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

#[derive(Debug, Default)]
pub struct Descriptor {
    pub width: usize,
    pub height: usize,
    pub resolution_x: usize,
    pub resolution_y: usize,
    pub resolution_unit: usize,
    pub orientation: Orientation,
    pub creation_date: Option<DateTime<Utc>>,
    pub original_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub copyright: Option<String>,
}

impl ExifExtractable for Descriptor {
    fn extract_from(
        &mut self,
        metadata: &little_exif::metadata::Metadata,
        tags: &[little_exif::exif_tag::ExifTag],
    ) -> Result<(), crate::error::CoreError> {
        self.width = get_tag_value::<Vec<u32>>(&tags[0], &metadata)?[0] as usize;
        self.height = get_tag_value::<Vec<u32>>(&tags[1], &metadata)?[0] as usize;
        Ok(())
    }
}

pub fn get_descriptor(metadata: &Metadata) -> Result<Descriptor, CoreError> {
    let mut descriptor = Descriptor::default();
    Descriptor::extract_from(
        &mut descriptor,
        metadata,
        &vec![
            ExifTag::ImageWidth(Vec::new()),
            ExifTag::ImageHeight(Vec::new()),
            ExifTag::ImageDescription(String::new()),
            ExifTag::XResolution(Vec::new()),
            ExifTag::YResolution(Vec::new()),
            ExifTag::ResolutionUnit(Vec::new()),
            ExifTag::Orientation(Vec::new()),
            ExifTag::CreateDate(String::new()),
            ExifTag::DateTimeOriginal(String::new()),
            ExifTag::ModifyDate(String::new()),
            ExifTag::Copyright(String::new()),
        ],
    )?;
    Ok(descriptor)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    fn get_metadata(filename: &str) -> little_exif::metadata::Metadata {
        use std::path::Path;
        let image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../resources/img")
            .join(filename);
        little_exif::metadata::Metadata::new_from_path(&image_path).unwrap()
    }

    #[rstest]
    #[case("text_car_animal_no-gps.png")]
    #[case("text_icon_gps.jpg")]
    fn has_descriptor(#[case] filename: &str) -> Result<(), CoreError> {
        let metadata = get_metadata(filename);
        let descriptor = get_descriptor(&metadata)?;
        assert_eq!(descriptor.width, 1024);
        Ok(())
    }
}
