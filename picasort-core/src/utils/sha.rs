use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::error::CoreError;

pub fn get_file_uuid<P: AsRef<Path>>(path: P) -> Result<String, CoreError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "text_icon_gps_nofile.jpg",
        "75f5e4ce87df5e4477421440a0073b51ef4713824181786938c709af3ae0f302",
        false
    )]
    #[case(
        "text_icon_gps.jpg",
        "75f5e4ce87df5e4477421440a0073b51ef4713824181786938c709af3ae0f302",
        true
    )]
    fn has_gps_data(
        #[case] filename: &str,
        #[case] hash: &str,
        #[case] correct: bool,
    ) -> Result<(), CoreError> {
        use std::path::Path;
        let image_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../resources/img")
            .join(filename);
        let h = get_file_uuid(image_path);

        if correct {
            assert_eq!(h.unwrap(), hash);
        } else {
            assert!(matches!(h.unwrap_err(), CoreError::IO(_)));
        }
        Ok(())
    }
}
