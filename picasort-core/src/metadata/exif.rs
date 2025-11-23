// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use chrono::{NaiveDate, NaiveTime};
use little_exif::{
    exif_tag::ExifTag, metadata::Metadata, rational::uR64, u8conversion::U8conversion,
};

use crate::error::CoreError;
pub trait ExifExtractable {
    fn extract_from(&mut self, metadata: &Metadata, tags: &[ExifTag]) -> Result<(), CoreError>;
}

pub trait ExifOutput {
    type Output;
    fn convert(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output;
}

impl ExifOutput for String {
    type Output = Result<String, CoreError>;
    fn convert(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        if let Some(tag) = metadata.get_tag(exif_tag).next() {
            let endian = metadata.get_endian();
            let tag_value = String::from_utf8(tag.value_as_u8_vec(&endian))?;
            Ok(tag_value.replace("\0", ""))
        } else {
            Err(CoreError::EXIFTagNotFound())
        }
    }
}

impl ExifOutput for Vec<uR64> {
    type Output = Result<Vec<uR64>, CoreError>;
    fn convert(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        if let Some(tag) = metadata.get_tag(exif_tag).next() {
            let endian = metadata.get_endian();
            let tag_value = <Vec<uR64> as U8conversion<Vec<uR64>>>::from_u8_vec(
                &tag.value_as_u8_vec(&endian),
                &endian,
            );
            Ok(tag_value)
        } else {
            Err(CoreError::EXIFTagNotFound())
        }
    }
}

pub fn string_to_date(tag: &ExifTag, metadata: &Metadata) -> Result<NaiveDate, CoreError> {
    let date_str = String::convert(tag, metadata)?;
    Ok(NaiveDate::parse_from_str(&date_str, "%Y:%m:%d")?)
}

pub fn vec_to_time(tag: &ExifTag, metadata: &Metadata) -> Result<NaiveTime, CoreError> {
    let time_u64_vec = Vec::convert(tag, metadata)?;
    let time_str = format!(
        "{:?}:{:?}:{:?}",
        time_u64_vec[0].nominator, time_u64_vec[1].nominator, time_u64_vec[2].nominator
    );
    Ok(NaiveTime::parse_from_str(&time_str, "%H:%M:%S")?)
}

pub fn get_tag_value<T: U8conversion<T>>(
    tag: &ExifTag,
    metadata: &Metadata,
) -> Result<T, CoreError> {
    if let Some(tag) = metadata.get_tag(tag).next() {
        let endian = metadata.get_endian();
        let tag_value = <T>::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian);
        return Ok(tag_value);
    }
    Err(CoreError::InvalidEXIFConversion(
        "Cannot get tag from metadata".to_string(),
    ))
}
