// Copyright (c) 2024 Lemur-Catta.org
// Author: Sylvain Gubian <sgubian@lemur-catta.org>

use std::fmt::Debug;

use crate::{
    DynamicGetSet,
    error::CoreError,
    metadata::{basics::Orientation, gps::GPSCoord},
};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use little_exif::{
    exif_tag::ExifTag, metadata::Metadata, rational::uR64, u8conversion::U8conversion,
};

#[derive(Debug)]
pub enum ExtractedValue {
    Text(String),
    Numbers(Vec<uR64>),
    UnsignedInt(usize),
    Date(NaiveDate),
    Time(NaiveTime),
    GPSCoord(GPSCoord),
    Orientation(Orientation),
    DateTime(DateTime<Utc>),
    // add more as needed
}

pub struct TagContext<'a> {
    pub destination: &'a str,
    pub main_tag: ExifTag,
    pub alternative: Option<ExifTag>,
    pub convert: fn(&ExifTag, &Metadata) -> Option<ExtractedValue>,
}

pub struct ExtractionSet<'a> {
    pub tags: Vec<TagContext<'a>>,
}

pub trait ExifExtractable {
    type Output;
    fn extract(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output;
}

pub trait ExifAssignable<'a>: DynamicGetSet + Debug {
    fn exif_set(&self) -> Option<ExtractionSet<'a>> {
        None
    }
    fn is_valid(&self) -> bool {
        true
    }
    fn assign(&mut self, metadata: &Metadata) -> Result<(), &'static str> {
        if let Some(es) = self.exif_set() {
            for tag in es.tags {
                let mut value = (tag.convert)(&tag.main_tag, metadata);
                if value.is_none()
                    && let Some(alt_tag) = tag.alternative
                {
                    value = (tag.convert)(&alt_tag, metadata);
                }

                match value {
                    Some(ExtractedValue::Text(s)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(s)))?;
                    }
                    Some(ExtractedValue::Time(t)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(t)))?;
                    }
                    Some(ExtractedValue::Numbers(n)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(n)))?;
                    }
                    Some(ExtractedValue::Date(d)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(d)))?;
                    }
                    Some(ExtractedValue::UnsignedInt(i)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(i)))?;
                    }
                    Some(ExtractedValue::GPSCoord(c)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(c)))?;
                    }
                    Some(ExtractedValue::Orientation(o)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(o)))?;
                    }
                    Some(ExtractedValue::DateTime(dt)) => {
                        self.set_field_by_name(tag.destination, Box::new(Some(dt)))?;
                    }
                    None => (),
                }
            }
        }
        Ok(())
    }
}

fn get_tag_value<T: U8conversion<T>>(tag: &ExifTag, metadata: &Metadata) -> Result<T, CoreError> {
    if let Some(tag) = metadata.get_tag(tag).next() {
        let endian = metadata.get_endian();
        let tag_value = <T>::from_u8_vec(&tag.value_as_u8_vec(&endian), &endian);
        return Ok(tag_value);
    }
    Err(CoreError::EXIFTagNotFound())
}

pub fn extract_unsigned_int32(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    let Some(v) = Vec::<u32>::extract(tag, meta) else {
        return None;
    };
    let Some(value) = v.into_iter().next() else {
        return None;
    };
    Some(ExtractedValue::UnsignedInt(value as usize))
}

pub fn extract_orientation(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    let Some(v) = Vec::<u16>::extract(tag, meta) else {
        return None;
    };
    let Some(value) = v.into_iter().next() else {
        return None;
    };
    Some(ExtractedValue::Orientation(Orientation::from_code(value)))
}

pub fn extract_unsigned_int16(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    let Some(v) = Vec::<u16>::extract(tag, meta) else {
        return None;
    };
    let Some(value) = v.into_iter().next() else {
        return None;
    };
    Some(ExtractedValue::UnsignedInt(value as usize))
}

pub fn extract_string(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    String::extract(tag, meta).map(ExtractedValue::Text)
}

pub fn extract_numbers(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    Vec::<uR64>::extract(tag, meta).map(ExtractedValue::Numbers)
}

pub fn extract_naive_date(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    NaiveDate::extract(tag, meta).map(ExtractedValue::Date)
}

pub fn extract_utc_datetime(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    DateTime::<Utc>::extract(tag, meta).map(ExtractedValue::DateTime)
}

pub fn extract_naive_time(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    NaiveTime::extract(tag, meta).map(ExtractedValue::Time)
}

pub fn extract_gps_coord(tag: &ExifTag, meta: &Metadata) -> Option<ExtractedValue> {
    if let Some(v) = Vec::<uR64>::extract(tag, meta) {
        let mut coord = GPSCoord::default();
        if v.len() != 3 {
            return None;
        }
        coord.deg = v[0].nominator as usize;
        coord.min = v[1].nominator as usize;
        coord.sec = v[2].nominator as f64 / v[2].denominator as f64;
        return Some(ExtractedValue::GPSCoord(coord));
    }
    return None;
}

impl ExifExtractable for String {
    type Output = Option<String>;
    fn extract(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        let Ok(tag_value) = get_tag_value::<String>(exif_tag, metadata) else {
            return None;
        };
        Some(tag_value.replace("\0", ""))
    }
}

impl<T> ExifExtractable for Vec<T>
where
    // T: U8conversion<T>,
    Vec<T>: U8conversion<Vec<T>>,
{
    type Output = Option<Vec<T>>;
    fn extract(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        let Ok(value) = get_tag_value::<Vec<T>>(exif_tag, metadata) else {
            return None;
        };
        Some(value)
    }
}

impl ExifExtractable for NaiveDate {
    type Output = Option<NaiveDate>;
    fn extract(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        let Some(date_str) = String::extract(exif_tag, metadata) else {
            return None;
        };
        Some(NaiveDate::parse_from_str(&date_str, "%Y:%m:%d").unwrap_or_default())
    }
}

impl ExifExtractable for DateTime<Utc> {
    type Output = Option<DateTime<Utc>>;
    fn extract(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        let Some(datetime) = String::extract(exif_tag, metadata) else {
            return None;
        };
        match NaiveDateTime::parse_from_str(&datetime, "%Y:%m:%d %H:%M:%S") {
            Ok(dt) => return Some(dt.and_utc()),
            Err(_) => return None,
        }
    }
}

impl ExifExtractable for NaiveTime {
    type Output = Option<NaiveTime>;
    fn extract(exif_tag: &ExifTag, metadata: &Metadata) -> Self::Output {
        let Some(time_u64_vec) = <Vec<uR64>>::extract(exif_tag, metadata) else {
            return None;
        };
        let time_str = format!(
            "{:?}:{:?}:{:?}",
            time_u64_vec[0].nominator, time_u64_vec[1].nominator, time_u64_vec[2].nominator
        );
        let Ok(nt) = NaiveTime::parse_from_str(&time_str, "%H:%M:%S") else {
            return None;
        };
        Some(nt)
    }
}
