use tempfile::{NamedTempFile, TempPath};

pub fn make_temp_with_ext(ext: &str) -> Result<(TempPath, String), String> {
    let file = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    let path = file.into_temp_path();
    // Append extension so ffmpeg knows format
    let path_str = format!("{}.{}", path.to_str().unwrap(), ext);
    Ok((path, path_str))
}
