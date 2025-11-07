// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use crate::metadata::exif::{ExifExtractable, ExifOutput};
use crate::try_assert;
use chrono::{NaiveDate, NaiveTime};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::rational::uR64;
use little_exif::u8conversion::U8conversion;

use crate::error::CoreError;

#[derive(Debug, Default)]
pub struct GPSCoord {
    reference: String,
    deg: u32,
    min: u32,
    sec: f32,
}

#[derive(Debug, Default)]
pub struct GPSTiming {
    time_stamp: NaiveTime,
    date_stamp: NaiveDate,
}

#[derive(Debug, Default)]
pub struct GPSData {
    pub latitude: GPSCoord,
    pub longitude: GPSCoord,
    pub time: GPSTiming,
}

impl ExifExtractable for GPSCoord {
    fn extract_from(&mut self, metadata: &Metadata, tags: &[ExifTag]) -> Result<(), CoreError> {
        try_assert!(
            tags.len() == 2,
            CoreError::InvalidGPSData(
                "Incorrect number of arguments for coordinates extraction.".into(),
            )
        );
        if let Some(tag) = metadata.get_tag(&tags[0]).next() {
            let endian = metadata.get_endian();
            let tag_value = <Vec<uR64> as U8conversion<Vec<uR64>>>::from_u8_vec(
                &tag.value_as_u8_vec(&endian),
                &endian,
            );
            self.deg = tag_value[0].nominator;
            self.min = tag_value[1].nominator;
            self.sec = tag_value[2].nominator as f32 / tag_value[2].denominator as f32;
        } else {
            return Err(CoreError::InvalidGPSData("Tag not found".into()));
        }
        self.reference = String::convert(&tags[1], metadata)?;
        Ok(())
    }
}

impl ExifExtractable for GPSTiming {
    fn extract_from(&mut self, metadata: &Metadata, tags: &[ExifTag]) -> Result<(), CoreError> {
        try_assert!(
            tags.len() == 2,
            CoreError::InvalidGPSData(
                "Incorrect number of arguments for Timing extraction.".into(),
            )
        );
        let date_str = String::convert(&tags[0], metadata)?;
        let time_u64_vec = Vec::convert(&tags[1], metadata)?;

        self.date_stamp = NaiveDate::parse_from_str(&date_str, "%Y:%m:%d")?;
        self.time_stamp = {
            let time_str = format!(
                "{:?}:{:?}:{:?}",
                time_u64_vec[0].nominator, time_u64_vec[1].nominator, time_u64_vec[2].nominator
            );
            NaiveTime::parse_from_str(&time_str, "%H:%M:%S")?
        };
        Ok(())
    }
}

pub fn get_gps_data(metadata: &Metadata) -> Result<GPSData, CoreError> {
    let mut gps_data = GPSData::default();
    GPSCoord::extract_from(
        &mut gps_data.latitude,
        metadata,
        &vec![
            ExifTag::GPSLatitude(Vec::new()),
            ExifTag::GPSLatitudeRef(String::new()),
        ],
    )?;
    GPSCoord::extract_from(
        &mut gps_data.longitude,
        metadata,
        &vec![
            ExifTag::GPSLongitude(Vec::new()),
            ExifTag::GPSLongitudeRef(String::new()),
        ],
    )?;
    GPSTiming::extract_from(
        &mut gps_data.time,
        metadata,
        &vec![
            ExifTag::GPSDateStamp(String::new()),
            ExifTag::GPSTimeStamp(Vec::new()),
        ],
    )?;

    Ok(gps_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use little_exif::exif_tag::ExifTag;
    use rstest::rstest;

    use crate::{error::CoreError, metadata::gps::get_gps_data};
    fn get_metadata(filename: &str) -> little_exif::metadata::Metadata {
        use std::path::Path;
        let image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../resources/img")
            .join(filename);
        little_exif::metadata::Metadata::new_from_path(&image_path).unwrap()
    }

    #[rstest]
    #[case("text_car_animal_no-gps.png", false, "", 0, "")]
    #[case("text_icon_gps.jpg", true, "N", 45, "2024-10-29")]
    fn has_gps_data(
        #[case] filename: &str,
        #[case] expected: bool,
        #[case] reference: &str,
        #[case] deg: u32,
        #[case] date: &str,
    ) -> Result<(), CoreError> {
        let metadata = get_metadata(filename);
        let gps_data = get_gps_data(&metadata);

        if expected {
            if let Ok(data) = gps_data {
                assert!(data.latitude.reference == reference);
                assert!(data.latitude.deg == deg);
                assert!(data.time.date_stamp.to_string() == date);
            } else {
                panic!("No GPS data found")
            };
        } else {
            let err = gps_data.unwrap_err();
            assert!(matches!(err, CoreError::InvalidGPSData(_)));
        }
        Ok(())
    }

    #[rstest]
    #[case("text_icon_gps.jpg", &vec![ExifTag::GPSDateStamp(String::new())], false)]
    #[case("text_icon_gps.jpg", &vec![ExifTag::GPSLatitude(Vec::new()), ExifTag::GPSLatitudeRef(String::new()),], true)]
    fn must_convert_coord_data(
        #[case] filename: &str,
        #[case] tags: &[ExifTag],
        #[case] ok_tags: bool,
    ) -> Result<(), CoreError> {
        use crate::metadata::gps::GPSData;

        let metadata = get_metadata(filename);
        let mut gps_data = GPSData::default();
        let result = GPSCoord::extract_from(&mut gps_data.latitude, &metadata, tags);
        if ok_tags {
            assert!(result.is_ok())
        } else {
            assert!(matches!(result.unwrap_err(), CoreError::InvalidGPSData(_)));
        }
        Ok(())
    }

    #[rstest]
    #[case("text_icon_gps.jpg", &vec![ExifTag::GPSLatitude(Vec::new())], false)]
    #[case("text_icon_gps.jpg", &vec![ExifTag::GPSDateStamp(String::new()), ExifTag::GPSTimeStamp(Vec::new())], true)]
    fn must_convert_time_data(
        #[case] filename: &str,
        #[case] tags: &[ExifTag],
        #[case] ok_tags: bool,
    ) -> Result<(), CoreError> {
        use crate::metadata::gps::GPSData;

        let metadata = get_metadata(filename);
        let mut gps_data = GPSData::default();
        let result = GPSTiming::extract_from(&mut gps_data.time, &metadata, tags);
        if ok_tags {
            assert!(result.is_ok())
        } else {
            assert!(matches!(result.unwrap_err(), CoreError::InvalidGPSData(_)));
        }
        Ok(())
    }
}
