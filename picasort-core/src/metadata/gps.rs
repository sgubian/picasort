// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::DynamicGetSet;
use crate::metadata::exif::{
    ExifAssignable, ExtractionSet, TagContext, extract_gps_coord, extract_naive_date,
    extract_naive_time, extract_string,
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
    fn is_valid(&self) -> bool {
        if let Some(lat) = &self.latitude_ref
            && lat.as_str() != "N"
            && lat.as_str() != "S"
        {
            return false;
        }
        if let Some(long) = &self.longitude_ref
            && long.as_str() != "O"
            && long.as_str() != "E"
        {
            return false;
        }
        if self.latitude.is_none() || self.longitude.is_none() {
            return false;
        }
        true
    }

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
                    convert: extract_naive_time,
                },
                TagContext {
                    destination: "date",
                    main_tag: ExifTag::GPSDateStamp(String::new()),
                    alternative: None,
                    convert: extract_naive_date,
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use chrono::NaiveTime;
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
    #[case(
        "text_car_animal_no-gps.png",
        "latitude",
        None,
        None,
        None,
        None,
        None,
        None
    )]
    #[case(
        "text_icon_gps.jpg",
        "latitude",
        Some("N".to_string()),
        Some(45),
        Some(45),
        Some(37.05),
        Some("11:33:25"),
        Some("2024-10-29"),
    )]
    #[case(
        "text_icon_gps.jpg",
        "longitude",
        Some("E".to_string()),
        Some(4),
        Some(51),
        Some(20.96),
        Some("11:33:25"),
        Some("2024-10-29"),
    )]
    fn has_gps_coord(
        #[case] filename: &str,
        #[case] direction: &str,
        #[case] reference: Option<String>,
        #[case] deg: Option<usize>,
        #[case] min: Option<usize>,
        #[case] sec: Option<f64>,
        #[case] time: Option<&str>,
        #[case] date: Option<&str>,
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
        if gps_data.time.is_some() {
            assert_eq!(
                gps_data.time,
                Some(NaiveTime::parse_from_str(time.unwrap_or_default(), "%H:%M:%S").unwrap())
            );
        }
        if gps_data.date.is_some() {
            assert_eq!(
                gps_data.date,
                Some(NaiveDate::parse_from_str(date.unwrap_or_default(), "%Y-%m-%d").unwrap())
            );
        }
    }

    #[rstest]
    #[case("text_car_animal_no-gps.png", false)]
    #[case("text_icon_gps.jpg", true)]
    fn has_validity_check(#[case] filename: &str, #[case] expected: bool) {
        use crate::metadata::gps::GPSData;

        let metadata = get_metadata(filename);
        let mut gps_data = GPSData::default();
        let res = gps_data.assign(&metadata);
        if res.is_err() {
            panic!("Error when assigning");
        }
        assert_eq!(gps_data.is_valid(), expected);
    }
}
