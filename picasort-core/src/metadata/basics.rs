// Copyright (c) 2025 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::DynamicGetSet;
use crate::metadata::exif::{
    ExifAssignable, ExtractionSet, TagContext, extract_string, extract_unsigned_int,
};
use chrono::{DateTime, Utc};

use little_exif::exif_tag::ExifTag;

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
                    convert: extract_unsigned_int,
                },
                TagContext {
                    destination: "height",
                    main_tag: ExifTag::ImageHeight(Vec::new()),
                    alternative: Some(ExifTag::ExifImageHeight(Vec::new())),
                    convert: extract_unsigned_int,
                },
                TagContext {
                    destination: "description",
                    main_tag: ExifTag::ImageDescription(String::new()),
                    alternative: None,
                    convert: extract_string,
                },
                // TagContext {
                //     destination: "resolution_x",
                //     main_tag: ExifTag::XResolution(Vec::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "resolution_y",
                //     main_tag: ExifTag::YResolution(Vec::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "resolution_unit",
                //     main_tag: ExifTag::ResolutionUnit(Vec::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "orientation",
                //     main_tag: ExifTag::Orientation(Vec::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "creation_date",
                //     main_tag: ExifTag::CreateDate(String::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "original_date",
                //     main_tag: ExifTag::DateTimeOriginal(String::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "modification_date",
                //     main_tag: ExifTag::ModifyDate(String::new()),
                //     alternative: None,
                // },
                // TagContext {
                //     destination: "copyright",
                //     main_tag: ExifTag::Copyright(String::new()),
                //     alternative: None,
                // },
            ],
        })
    }
}

#[cfg(test)]
mod tests {

    // use super::*;
    use rstest::rstest;

    use crate::metadata::{basics::Basics, exif::ExifAssignable};

    fn get_metadata(filename: &str) -> little_exif::metadata::Metadata {
        use std::path::Path;
        let image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../resources/img")
            .join(filename);
        little_exif::metadata::Metadata::new_from_path(&image_path).unwrap()
    }

    #[rstest]
    #[case("text_car_animal_no-gps.png", 1024, 769, None)]
    #[case("text_icon_gps.jpg", 3840, 2160, None)]
    fn has_basics(
        #[case] filename: &str,
        #[case] width: usize,
        #[case] height: usize,
        #[case] desc: Option<String>,
    ) {
        let metadata = get_metadata(filename);
        let mut basics = Basics::default();
        let _ = basics.assign(&metadata);
        assert_eq!(basics.width, Some(width));
        assert_eq!(basics.height, Some(height));
        assert_eq!(basics.height, Some(height));
        assert_eq!(basics.desciption, desc);
    }
}
