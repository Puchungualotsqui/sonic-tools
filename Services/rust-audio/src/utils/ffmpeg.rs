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

pub fn probe_bitrate(input_bytes: &[u8], _ext: &str) -> Result<i32, String> {
    // 1) Create temp file and write bytes
    let tmp = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    std::fs::write(tmp.path(), input_bytes).map_err(|e| format!("write tmp: {}", e))?;

    // 2) Convert to TempPath: closes handle, keeps file on disk, will delete on drop
    let tmp_path = tmp.into_temp_path();

    // 3) Run ffprobe on the path
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-of",
            "csv=p=0",
            "-show_entries",
            "format=bit_rate",
            tmp_path.as_os_str().to_str().ok_or("tmp path utf-8")?,
        ])
        .output()
        .map_err(|e| format!("ffprobe exec: {}", e))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        // Temp file auto-deletes here on drop
        return Err(format!("ffprobe failed: {}", err));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Some files return "N/A"; handle that if needed
    let bps: i32 = stdout
        .trim()
        .parse()
        .map_err(|_| format!("Could not parse bitrate from '{}'", stdout.trim()))?;

    // 4) Explicit delete now (optional). If you omit this, it deletes on drop anyway.
    tmp_path.close().map_err(|e| format!("delete tmp: {}", e))?;

    Ok(bps)
}

pub fn probe_duration(input_bytes: &[u8]) -> Result<f32, String> {
    let tmp = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    std::fs::write(tmp.path(), input_bytes).map_err(|e| format!("write tmp: {}", e))?;
    let tmp_path = tmp.into_temp_path();

    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-of",
            "csv=p=0",
            "-show_entries",
            "format=duration",
            tmp_path.as_os_str().to_str().ok_or("tmp path utf-8")?,
        ])
        .output()
        .map_err(|e| format!("ffprobe exec: {}", e))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ffprobe failed: {}", err));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let secs: f32 = stdout
        .trim()
        .parse()
        .map_err(|_| format!("Could not parse duration from '{}'", stdout.trim()))?;

    tmp_path.close().map_err(|e| format!("delete tmp: {}", e))?;

    Ok(secs)
}
