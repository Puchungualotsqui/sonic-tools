use crate::utils::conversion::convert_file;

pub fn trim_file(
    input_bytes: Vec<u8>,
    output_format: &str,
    start_sec: Option<i32>,
    end_sec: Option<i32>,
    action: &str, // "keep" or "remove"
) -> Result<Vec<u8>, String> {
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    // Sanity checks
    if let (Some(s), Some(e)) = (start_sec, end_sec) {
        if e <= s {
            return Err("end_sec must be greater than start_sec".into());
        }
    }
    if action != "keep" && action != "remove" {
        return Err("Invalid action (must be 'keep' or 'remove')".into());
    }

    // 1) Decode input -> WAV (robust intermediate)
    // We don't know the input extension here; pass None.
    let wav_bytes = convert_file(input_bytes, "wav", 0, None)?;

    // Persist WAV to temp file for ffmpeg
    let tmp_wav_in = NamedTempFile::new().map_err(|e| format!("tmpfile wav in: {}", e))?;
    fs::write(tmp_wav_in.path(), &wav_bytes).map_err(|e| format!("write wav in: {}", e))?;
    let wav_in_path = tmp_wav_in.into_temp_path();

    // 2) Prepare temp output (still WAV after trimming/concatenation)
    let tmp_wav_out = NamedTempFile::new().map_err(|e| format!("tmpfile wav out: {}", e))?;
    let wav_out_path = tmp_wav_out.into_temp_path();

    match action {
        "keep" => {
            // Single segment: [start, end) copied out of WAV
            let mut args: Vec<String> = vec![
                "-y".into(),
                "-hide_banner".into(),
                "-loglevel".into(),
                "error".into(),
                "-i".into(),
                wav_in_path.to_str().ok_or("bad wav_in_path")?.into(),
            ];

            if let Some(s) = start_sec {
                args.push("-ss".into());
                args.push(s.to_string());
            }
            if let Some(e) = end_sec {
                args.push("-to".into());
                args.push(e.to_string());
            }

            // copy PCM samples to new WAV
            args.extend([
                "-c".into(),
                "copy".into(),
                "-f".into(),
                "wav".into(),
                wav_out_path.to_str().ok_or("bad wav_out_path")?.into(),
            ]);

            let out = Command::new("ffmpeg")
                .args(&args)
                .output()
                .map_err(|e| format!("ffmpeg trim keep exec: {}", e))?;
            if !out.status.success() {
                return Err(format!(
                    "ffmpeg trim keep failed: {}",
                    String::from_utf8_lossy(&out.stderr)
                ));
            }

            let trimmed_wav =
                fs::read(&wav_out_path).map_err(|e| format!("read trimmed wav: {}", e))?;
            // 3) Re-encode to requested container/codec using your central plan
            let final_bytes = convert_file(trimmed_wav, output_format, 0, Some("wav"))?;
            Ok(final_bytes)
        }

        "remove" => {
            use tempfile::TempPath;

            let mut parts: Vec<String> = Vec::new();
            let mut keepers: Vec<TempPath> = Vec::new(); // keep temp files alive

            // Segment 1: before start
            if let Some(s) = start_sec {
                let p1 = NamedTempFile::new().map_err(|e| format!("tmpfile part1: {}", e))?;
                let p1_path = p1.into_temp_path();
                let args = vec![
                    "-y".into(),
                    "-hide_banner".into(),
                    "-loglevel".into(),
                    "error".into(),
                    "-i".into(),
                    wav_in_path.to_str().ok_or("bad wav_in_path")?.into(),
                    "-to".into(),
                    s.to_string(),
                    "-c".into(),
                    "copy".into(),
                    "-f".into(),
                    "wav".into(),
                    p1_path.to_str().ok_or("bad p1")?.into(),
                ];
                let out = Command::new("ffmpeg")
                    .args(&args)
                    .output()
                    .map_err(|e| format!("ffmpeg part1 exec: {}", e))?;
                if !out.status.success() {
                    return Err(format!(
                        "ffmpeg part1 failed: {}",
                        String::from_utf8_lossy(&out.stderr)
                    ));
                }
                parts.push(p1_path.to_string_lossy().into_owned());
                keepers.push(p1_path);
            }

            // Segment 2: after end
            if let Some(e) = end_sec {
                let p2 = NamedTempFile::new().map_err(|e| format!("tmpfile part2: {}", e))?;
                let p2_path = p2.into_temp_path();
                let args = vec![
                    "-y".into(),
                    "-hide_banner".into(),
                    "-loglevel".into(),
                    "error".into(),
                    "-i".into(),
                    wav_in_path.to_str().ok_or("bad wav_in_path")?.into(),
                    "-ss".into(),
                    e.to_string(),
                    "-c".into(),
                    "copy".into(),
                    "-f".into(),
                    "wav".into(),
                    p2_path.to_str().ok_or("bad p2")?.into(),
                ];
                let out = Command::new("ffmpeg")
                    .args(&args)
                    .output()
                    .map_err(|e| format!("ffmpeg part2 exec: {}", e))?;
                if !out.status.success() {
                    return Err(format!(
                        "ffmpeg part2 failed: {}",
                        String::from_utf8_lossy(&out.stderr)
                    ));
                }
                parts.push(p2_path.to_string_lossy().into_owned());
                keepers.push(p2_path);
            }

            if parts.is_empty() {
                // If neither start nor end was provided, there's nothing to remove
                return Err("no parts to keep after removal".into());
            }

            // Concat list file (for WAV parts)
            let concat_list =
                NamedTempFile::new().map_err(|e| format!("tmpfile concat list: {}", e))?;
            let concat_path = concat_list.into_temp_path();
            {
                let mut f = fs::File::create(&concat_path)
                    .map_err(|e| format!("create concat list: {}", e))?;
                for p in &parts {
                    // escape single quotes
                    let esc = p.replace('\'', "'\\''");
                    writeln!(f, "file '{}'", esc)
                        .map_err(|e| format!("write concat list: {}", e))?;
                }
            }

            // Concat WAV parts -> single WAV
            let out = Command::new("ffmpeg")
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
                    concat_path.to_str().ok_or("bad concat path")?,
                    "-c",
                    "copy",
                    "-f",
                    "wav",
                    wav_out_path.to_str().ok_or("bad wav_out_path")?,
                ])
                .output()
                .map_err(|e| format!("ffmpeg concat exec: {}", e))?;
            if !out.status.success() {
                return Err(format!(
                    "ffmpeg concat failed: {}",
                    String::from_utf8_lossy(&out.stderr)
                ));
            }

            // 3) Re-encode concatenated WAV to the requested format
            let merged_wav =
                fs::read(&wav_out_path).map_err(|e| format!("read merged wav: {}", e))?;
            let final_bytes = convert_file(merged_wav, output_format, 0, Some("wav"))?;
            Ok(final_bytes)
        }

        _ => unreachable!(),
    }
}
