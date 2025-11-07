use crate::error::CoreError;

pub struct ThumbnailInfo<'a> {
    file_path: &'a str,
    ration: u16,
}

pub fn generate_thumbnails() -> Result<(), CoreError> {
    Ok(())
}
