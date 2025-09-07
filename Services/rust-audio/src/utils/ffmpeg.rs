use std::io::Write;
use std::process::{Command, Stdio};

pub fn compress_file(
    input_bytes: Vec<u8>,
    output_format: &str,
    bitrate: Option<i32>,
) -> Result<Vec<u8>, String> {
    println!(
        "Running ffmpeg for format={}, bitrate={:?}",
        output_format, bitrate
    );
    let mut child = Command::new("ffmpeg")
        .args([
            "-i",
            "pipe:0",
            "-b:a",
            &format!("{}k", bitrate.unwrap_or(128)),
            "-f",
            output_format,
            "pipe:1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn ffmpeg: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&input_bytes)
            .map_err(|e| format!("Failed to write to ffmpeg stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("ffmpeg failed: {}", stderr_msg); // <-- log to console
        return Err(format!("ffmpeg failed: {}", stderr_msg));
    }

    Ok(output.stdout)
}

pub fn probe_bitrate(input_bytes: &[u8]) -> Result<i32, String> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("ffprobe")
        .args(&[
            "-i",
            "pipe:0",
            "-show_entries",
            "format=bit_rate",
            "-v",
            "quiet",
            "-of",
            "csv=p=0",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn ffprobe: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input_bytes)
            .map_err(|e| format!("Failed to write to ffprobe stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to run ffprobe: {}", e))?;

    if !output.status.success() {
        return Err("ffprobe failed".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .trim()
        .parse::<i32>()
        .map_err(|_| "Could not parse bitrate".into())
}

pub fn probe_duration(input_bytes: &[u8]) -> Result<f32, String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = Command::new("ffprobe")
        .args(&[
            "-i",
            "pipe:0",
            "-show_entries",
            "format=duration",
            "-v",
            "quiet",
            "-of",
            "csv=p=0",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn ffprobe: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input_bytes)
            .map_err(|e| format!("Failed to write to ffprobe stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to run ffprobe: {}", e))?;

    if !output.status.success() {
        return Err("ffprobe failed".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .trim()
        .parse::<f32>()
        .map_err(|_| "Could not parse duration".into())
}
