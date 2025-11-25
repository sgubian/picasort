// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::DynamicGetSet;
use crate::metadata::exif::{
    ExifAssignable, ExtractionSet, TagContext, extract_date, extract_gps_coord, extract_string,
    extract_time,
};
use chrono::{NaiveDate, NaiveTime};
use little_exif::exif_tag::ExifTag;

#[derive(Debug, Default)]
pub struct GPSCoord {
    pub deg: usize,
    pub min: usize,
    pub sec: f64,
}

#[derive(Debug, Default, DynamicGetSet)]
pub struct GPSData {
    pub latitude_ref: Option<String>,
    pub latitude: Option<GPSCoord>,
    pub longitude_ref: Option<String>,
    pub longitude: Option<GPSCoord>,
    pub time: Option<NaiveTime>,
    pub date: Option<NaiveDate>,
}

impl<'a> ExifAssignable<'a> for GPSData {
    fn exif_set(&self) -> Option<ExtractionSet<'a>> {
        Some(ExtractionSet {
            tags: vec![
                TagContext {
                    destination: "latitude_ref",
                    main_tag: ExifTag::GPSLatitudeRef(String::new()),
                    alternative: None,
                    convert: extract_string,
                },
                TagContext {
                    destination: "latitude",
                    main_tag: ExifTag::GPSLatitude(Vec::new()),
                    alternative: None,
                    convert: extract_gps_coord,
                },
                TagContext {
                    destination: "longitude_ref",
                    main_tag: ExifTag::GPSLongitudeRef(String::new()),
                    alternative: None,
                    convert: extract_string,
                },
                TagContext {
                    destination: "longitude",
                    main_tag: ExifTag::GPSLongitude(Vec::new()),
                    alternative: None,
                    convert: extract_gps_coord,
                },
                TagContext {
                    destination: "time",
                    main_tag: ExifTag::GPSTimeStamp(Vec::new()),
                    alternative: None,
                    convert: extract_time,
                },
                TagContext {
                    destination: "date",
                    main_tag: ExifTag::GPSDateStamp(String::new()),
                    alternative: None,
                    convert: extract_date,
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::metadata::exif::ExifAssignable;

    fn get_metadata(filename: &str) -> little_exif::metadata::Metadata {
        use std::path::Path;
        let image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../resources/img")
            .join(filename);
        little_exif::metadata::Metadata::new_from_path(&image_path).unwrap()
    }

    #[rstest]
    #[case("text_car_animal_no-gps.png", "latitude", None, None, None, None)]
    #[case(
        "text_icon_gps.jpg",
        "latitude",
        Some("N".to_string()),
        Some(45),
        Some(45),
        Some(37.05)
    )]
    #[case(
        "text_icon_gps.jpg",
        "longitude",
        Some("E".to_string()),
        Some(4),
        Some(51),
        Some(20.96)
    )]
    fn has_gps_coord(
        #[case] filename: &str,
        #[case] direction: &str,
        #[case] reference: Option<String>,
        #[case] deg: Option<usize>,
        #[case] min: Option<usize>,
        #[case] sec: Option<f64>,
    ) {
        use crate::metadata::gps::GPSData;

        let metadata = get_metadata(filename);
        let mut gps_data = GPSData::default();
        let res = gps_data.assign(&metadata);
        if res.is_err() {
            panic!("Error when assigning");
        }
        let mut coord = gps_data.latitude.unwrap_or_default();
        let mut refe = gps_data.latitude_ref.unwrap_or_default();
        if direction != "latitude" {
            coord = gps_data.longitude.unwrap_or_default();
            refe = gps_data.longitude_ref.unwrap_or_default();
        }

        assert_eq!(refe, reference.unwrap_or_default());
        assert_eq!(coord.deg, deg.unwrap_or_default());
        assert_eq!(coord.min, min.unwrap_or_default());
        assert_eq!(coord.sec, sec.unwrap_or_default());
    }
}
