use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
enum VttPathError {
    InvalidPath,
    VttNotFound,
}

/// Given a path, find its related .vtt file.
fn locate(video_path: &Path) -> Result<PathBuf, VttPathError> {
    let vtt_path = video_path.with_extension("vtt");
    if vtt_path.exists() { return Ok(vtt_path); }

    let file_name = video_path.file_name().ok_or(VttPathError::InvalidPath)?;
    let mut file_name = file_name.to_os_string();
    file_name.push(".vtt");
    let vtt_path = video_path.with_file_name(file_name);
    if vtt_path.exists() { return Ok(vtt_path); }

    Err(VttPathError::VttNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn doesnt_find_if_no_file() {
        let dir = tempdir().unwrap();

        let video_path = dir.path().join("test.mp4");
        File::create(&video_path).unwrap();

        let vtt_path = dir.path().join("test.asd");
        File::create(&vtt_path).unwrap();

        let result = locate(&video_path);
        let expected = Err(VttPathError::VttNotFound);
        assert_eq!(result, expected);
    }

    #[test]
    fn finds_replaced_extension() {
        let dir = tempdir().unwrap();

        let video_path = dir.path().join("test.mp4");
        File::create(&video_path).unwrap();

        let vtt_path = dir.path().join("test.vtt");
        File::create(&vtt_path).unwrap();

        let found = locate(&video_path).unwrap();
        assert_eq!(found, vtt_path);
    }

    #[test]
    fn finds_appended_extension() {
        let dir = tempdir().unwrap();

        let video_path = dir.path().join("test.mp4");
        File::create(&video_path).unwrap();

        let vtt_path = dir.path().join("test.mp4.vtt");
        File::create(&vtt_path).unwrap();

        let found = locate(&video_path).unwrap();
        assert_eq!(found, vtt_path);
    }

    #[test]
    fn prefers_replaced_extension() {
        let dir = tempdir().unwrap();

        let video_path = dir.path().join("test.mp4");
        File::create(&video_path).unwrap();

        let replaced_vtt_path = dir.path().join("test.vtt");
        File::create(&replaced_vtt_path).unwrap();

        let appended_vtt_path = dir.path().join("test.mp4.vtt");
        File::create(&appended_vtt_path).unwrap();

        let found = locate(&video_path).unwrap();
        assert_eq!(found, replaced_vtt_path);
    }
}
