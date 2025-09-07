use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn convert_file(
    input_bytes: Vec<u8>,
    output_format: &str,
    bitrate: i32,
) -> Result<Vec<u8>, String> {
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();
    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    let out_path = tmp_out.into_temp_path();
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-i", in_path.to_str().ok_or("bad in_path")?]);
    if bitrate > 0 {
        cmd.args(["-b:a", &format!("{}k", bitrate)]);
    }
    cmd.args([
        "-f",
        output_format,
        out_path.to_str().ok_or("bad out_path")?,
    ]);
    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;
    if !status.success() {
        return Err("ffmpeg failed".into());
    }
    let bytes = fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))?;
    Ok(bytes)
}
