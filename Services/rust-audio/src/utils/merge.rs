use crate::utils::conversion::convert_file;
use std::{fs, io::Write, process::Command};
use tempfile::NamedTempFile;

// Public: Merge sequentially and encode to requested format.
pub fn merge_sequential(
    inputs: Vec<(String, Vec<u8>)>,
    output_format: &str,
) -> Result<Vec<u8>, String> {
    if inputs.is_empty() {
        return Err("no inputs".into());
    }

    // Convert all inputs to WAV using your convert_file
    let mut normalized: Vec<tempfile::TempPath> = Vec::new();
    for (_name, data) in inputs {
        let wav_bytes = convert_file(data, "wav", 0)?; // 0 bitrate = default
        let tmp = NamedTempFile::new().map_err(|e| format!("tmpfile out: {}", e))?;
        fs::write(tmp.path(), &wav_bytes).map_err(|e| format!("write tmp out: {}", e))?;
        normalized.push(tmp.into_temp_path());
    }

    // Write concat list file
    let list_file = NamedTempFile::new().map_err(|e| format!("concat list tmp: {}", e))?;
    {
        let mut f = &list_file;
        for p in &normalized {
            let path = p.to_str().ok_or("bad temp path")?;
            writeln!(f, "file '{}'", path.replace('\'', "'\\''"))
                .map_err(|e| format!("write concat list: {}", e))?;
        }
    }
    let list_path = list_file.into_temp_path();

    // Output file
    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile out: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    // Encode to requested format
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            list_path.to_str().ok_or("list path utf-8")?,
            "-f",
            output_format,
            out_path.to_str().ok_or("bad out_path")?,
        ])
        .output()
        .map_err(|e| format!("ffmpeg exec: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ffmpeg failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(fs::read(&out_path).map_err(|e| format!("read merged: {}", e))?)
}
