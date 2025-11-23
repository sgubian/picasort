use crate::error::CoreError;

pub struct ThumbnailInfo<'a> {
    _file_path: &'a str,
    _ratio: u16,
}

pub fn generate_thumbnails() -> Result<(), CoreError> {
    Ok(())
}
