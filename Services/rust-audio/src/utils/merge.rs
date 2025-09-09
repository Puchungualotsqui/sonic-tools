use crate::utils::conversion::convert_file;
use std::{fs, io::Write, path::Path, process::Command};
use tempfile::NamedTempFile;

fn ext_of(name: &str) -> Option<&str> {
    Path::new(name).extension().and_then(|e| e.to_str())
}

/// Merge sequentially and encode to the requested `output_format`.
/// Supports AAC (.aac), M4A (AAC), and ALAC (.m4a) via convert_file's plan.
/// Bitrate is fixed to 0 here (meaning "default"), but you can add a parameter if needed.
pub fn merge_sequential(
    inputs: Vec<(String, Vec<u8>)>,
    output_format: &str,
) -> Result<Vec<u8>, String> {
    if inputs.is_empty() {
        return Err("no inputs".into());
    }

    // 1) Convert all inputs to WAV using the shared convert_file (pass original ext for sniffing)
    let mut wav_paths: Vec<tempfile::TempPath> = Vec::new();
    for (name, data) in inputs {
        let in_ext = ext_of(&name);
        let wav_bytes = convert_file(data, "wav", 0, in_ext)?; // 0 bitrate = default
        let tmp = NamedTempFile::new().map_err(|e| format!("tmpfile out: {}", e))?;
        fs::write(tmp.path(), &wav_bytes).map_err(|e| format!("write tmp out: {}", e))?;
        wav_paths.push(tmp.into_temp_path());
    }

    // 2) Build concat list file
    let list_file = NamedTempFile::new().map_err(|e| format!("concat list tmp: {}", e))?;
    {
        let mut f = list_file
            .as_file()
            .try_clone()
            .map_err(|e| format!("concat list open: {}", e))?;
        for p in &wav_paths {
            let path = p.to_str().ok_or("bad temp path")?;
            // Escape single quotes for concat demuxer line format
            writeln!(f, "file '{}'", path.replace('\'', "'\\''"))
                .map_err(|e| format!("write concat list: {}", e))?;
        }
    }
    let list_path = list_file.into_temp_path();

    // 3) Concat WAVs -> single WAV
    let merged_wav = NamedTempFile::new().map_err(|e| format!("tmpfile out: {}", e))?;
    let merged_wav_path = merged_wav.into_temp_path();

    // Safer to re-encode to a standard WAV (pcm_s16le) than try `-c copy`
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-hide_banner",
            "-loglevel",
            "error",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            list_path.to_str().ok_or("list path utf-8")?,
            "-c:a",
            "pcm_s16le",
            "-f",
            "wav",
            merged_wav_path.to_str().ok_or("bad merged_wav_path")?,
        ])
        .output()
        .map_err(|e| format!("ffmpeg exec: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ffmpeg failed (concat): {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // 4) Re-encode merged WAV to the requested output format using the shared plan
    let merged_wav_bytes =
        fs::read(&merged_wav_path).map_err(|e| format!("read merged wav: {}", e))?;

    // Reuse convert_file so AAC/M4A/ALAC mapping works consistently.
    // Pass Some("wav") so ffmpeg knows input type.
    let final_bytes = convert_file(merged_wav_bytes, output_format, 0, Some("wav"))?;

    Ok(final_bytes)
}
