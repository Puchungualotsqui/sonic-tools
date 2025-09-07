use std::process::Command;
use tempfile::NamedTempFile;

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
