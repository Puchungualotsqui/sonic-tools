use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn compress_file(
    input_bytes: Vec<u8>,
    output_format: &str,
    bitrate: Option<i32>,
) -> Result<Vec<u8>, String> {
    // 1) Create temp input file
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    // 2) Create temp output file
    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    // 3) Run ffmpeg: input → output
    let bitrate_arg = format!("{}k", bitrate.unwrap_or(128));
    let status = Command::new("ffmpeg")
        .args([
            "-y", // overwrite output
            "-i",
            in_path.to_str().ok_or("bad in_path")?,
            "-b:a",
            &bitrate_arg,
            "-f",
            output_format,
            out_path.to_str().ok_or("bad out_path")?,
        ])
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    // 4) Read compressed file
    let bytes = fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))?;

    // 5) Temp files auto-delete when paths drop
    Ok(bytes)
}
