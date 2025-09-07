use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn boost_file(input_bytes: Vec<u8>, output_format: &str, gain: i32) -> Result<Vec<u8>, String> {
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    let gain_arg = format!("volume={}dB", gain);
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            in_path.to_str().ok_or("bad in_path")?,
            "-af",
            &gain_arg,
            "-f",
            output_format,
            out_path.to_str().ok_or("bad out_path")?,
        ])
        .status()
        .map_err(|e| format!("ffmpeg error: {}", e))?;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))
}

pub fn normalize_file(input_bytes: Vec<u8>, output_format: &str) -> Result<Vec<u8>, String> {
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    // Use EBU R128 loudness normalization
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            in_path.to_str().ok_or("bad in_path")?,
            "-af",
            "loudnorm",
            "-f",
            output_format,
            out_path.to_str().ok_or("bad out_path")?,
        ])
        .status()
        .map_err(|e| format!("ffmpeg error: {}", e))?;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))
}
