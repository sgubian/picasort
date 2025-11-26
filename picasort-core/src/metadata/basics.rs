// Copyright (c) 2025 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::DynamicGetSet;
use crate::metadata::exif::{
    ExifAssignable, ExtractionSet, TagContext, extract_orientation, extract_string,
    extract_unsigned_int16, extract_unsigned_int32, extract_utc_datetime,
};
use chrono::{DateTime, Utc};

use little_exif::exif_tag::ExifTag;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Orientation {
    Normal = 1,
    FlippedHorizontally = 2,
    Rotated180Deg = 3,
    FlippedVertically = 4,
    Rotated90DegCCWFlippedVertically = 5,
    Rotated90DegCW = 6,
    Rotated90DegCCWPFlippedHorizontally = 7,
    Rotated90DegCCW = 8,
    Unknown = 9,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Normal
    }
}

impl Orientation {
    pub fn from_code(code: u16) -> Orientation {
        match code {
            1 => Orientation::Normal,
            2 => Orientation::FlippedHorizontally,
            3 => Orientation::Rotated180Deg,
            4 => Orientation::FlippedVertically,
            5 => Orientation::Rotated90DegCCWFlippedVertically,
            6 => Orientation::Rotated90DegCCW,
            7 => Orientation::Rotated90DegCCWPFlippedHorizontally,
            8 => Orientation::Rotated90DegCCW,
            _ => Orientation::Unknown,
        }
    }

    pub fn code(&self) -> u16 {
        *self as u16
    }
}

#[derive(Debug, Default, DynamicGetSet)]
pub struct Basics {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub desciption: Option<String>,
    pub resolution_x: Option<usize>,
    pub resolution_y: Option<usize>,
    pub resolution_unit: Option<usize>,
    pub orientation: Option<Orientation>,
    pub creation_date: Option<DateTime<Utc>>,
    pub original_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub copyright: Option<String>,
}

impl<'a> ExifAssignable<'a> for Basics {
    fn exif_set(&self) -> Option<ExtractionSet<'a>> {
        Some(ExtractionSet {
            tags: vec![
                TagContext {
                    destination: "width",
                    main_tag: ExifTag::ImageWidth(Vec::new()),
                    alternative: Some(ExifTag::ExifImageWidth(Vec::new())),
                    convert: extract_unsigned_int32,
                },
                TagContext {
                    destination: "height",
                    main_tag: ExifTag::ImageHeight(Vec::new()),
                    alternative: Some(ExifTag::ExifImageHeight(Vec::new())),
                    convert: extract_unsigned_int32,
                },
                TagContext {
                    destination: "description",
                    main_tag: ExifTag::ImageDescription(String::new()),
                    alternative: None,
                    convert: extract_string,
                },
                TagContext {
                    destination: "resolution_x",
                    main_tag: ExifTag::XResolution(Vec::new()),
                    alternative: None,
                    convert: extract_unsigned_int32,
                },
                TagContext {
                    destination: "resolution_y",
                    main_tag: ExifTag::YResolution(Vec::new()),
                    alternative: None,
                    convert: extract_unsigned_int32,
                },
                TagContext {
                    destination: "resolution_unit",
                    main_tag: ExifTag::ResolutionUnit(Vec::new()),
                    alternative: None,
                    convert: extract_unsigned_int16,
                },
                TagContext {
                    destination: "orientation",
                    main_tag: ExifTag::Orientation(Vec::new()),
                    alternative: None,
                    convert: extract_orientation,
                },
                TagContext {
                    destination: "creation_date",
                    main_tag: ExifTag::CreateDate(String::new()),
                    alternative: None,
                    convert: extract_utc_datetime,
                },
                TagContext {
                    destination: "original_date",
                    main_tag: ExifTag::DateTimeOriginal(String::new()),
                    alternative: None,
                    convert: extract_utc_datetime,
                },
                TagContext {
                    destination: "modification_date",
                    main_tag: ExifTag::ModifyDate(String::new()),
                    alternative: None,
                    convert: extract_utc_datetime,
                },
                TagContext {
                    destination: "copyright",
                    main_tag: ExifTag::Copyright(String::new()),
                    alternative: None,
                    convert: extract_string,
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::metadata::{
        basics::{Basics, Orientation},
        exif::ExifAssignable,
    };
    use chrono::DateTime;
    use rstest::rstest;

    fn get_metadata(filename: &str) -> little_exif::metadata::Metadata {
        use std::path::Path;
        let image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../resources/img")
            .join(filename);
        little_exif::metadata::Metadata::new_from_path(&image_path).unwrap()
    }

    #[rstest]
    #[case(
        "text_car_animal_no-gps.png",
        1024,
        769,
        None,
        350,
        350,
        3,
        Orientation::Normal,
        Some("2024-12-27T15:58:43Z"),
        Some("2024-12-27T15:58:43Z"),
        Some("2025-11-02T10:45:59Z")
    )]
    #[case(
        "text_icon_gps.jpg",
        3840,
        2160,
        None,
        72,
        72,
        2,
        Orientation::Rotated90DegCCW,
        Some("2024-10-28T20:35:03Z"),
        Some("2024-10-28T20:35:03Z"),
        Some("2024-10-28T20:35:03Z")
    )]
    fn has_basics(
        #[case] filename: &str,
        #[case] width: usize,
        #[case] height: usize,
        #[case] desc: Option<String>,
        #[case] xres: usize,
        #[case] yres: usize,
        #[case] res_unit: usize,
        #[case] orientation: Orientation,
        #[case] date_created: Option<&str>,
        #[case] date_original: Option<&str>,
        #[case] date_modification: Option<&str>,
    ) {
        let metadata = get_metadata(filename);
        let mut basics = Basics::default();
        let res = basics.assign(&metadata);
        if res.is_err() {
            panic!("Error when assigning");
        }
        assert_eq!(basics.width, Some(width));
        assert_eq!(basics.height, Some(height));
        assert_eq!(basics.height, Some(height));
        assert_eq!(basics.desciption, desc);
        assert_eq!(basics.resolution_x, Some(xres));
        assert_eq!(basics.resolution_y, Some(yres));
        assert_eq!(basics.resolution_unit, Some(res_unit));
        assert_eq!(basics.orientation, Some(orientation));

        if let Some(dc) = date_created {
            assert_eq!(
                basics.creation_date,
                Some(DateTime::parse_from_rfc3339(dc).unwrap().to_utc())
            );
        }
        if let Some(dori) = date_original {
            assert_eq!(
                basics.original_date,
                Some(DateTime::parse_from_rfc3339(dori).unwrap().to_utc())
            );
        }
        if let Some(dm) = date_modification {
            assert_eq!(
                basics.modification_date,
                Some(DateTime::parse_from_rfc3339(dm).unwrap().to_utc())
            );
        }
    }
}
