use crate::utils::temp::make_temp_with_ext;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;
use tempfile::TempPath;

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

pub fn trim_file(
    input_bytes: Vec<u8>,
    output_format: &str,
    start_sec: Option<i32>,
    end_sec: Option<i32>,
    action: &str,
) -> Result<Vec<u8>, String> {
    use std::fs;
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    // 1) Write input to temp file
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    // 2) Temp output file
    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    match action {
        "keep" => {
            let mut args: Vec<String> =
                vec!["-y".into(), "-i".into(), in_path.to_str().unwrap().into()];

            if let Some(start) = start_sec {
                args.push("-ss".into());
                args.push(start.to_string());
            }
            if let Some(end) = end_sec {
                args.push("-to".into());
                args.push(end.to_string());
            }

            args.push("-c".into());
            args.push("copy".into());
            args.push("-f".into());
            args.push(output_format.to_string());
            args.push(out_path.to_str().unwrap().to_string());

            let status = Command::new("ffmpeg")
                .args(&args)
                .status()
                .map_err(|e| format!("ffmpeg keep trim: {}", e))?;
            if !status.success() {
                return Err("ffmpeg failed during keep trim".into());
            }

            return fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e));
        }

        "remove" => {
            let mut parts: Vec<String> = Vec::new();
            let mut keepers: Vec<TempPath> = Vec::new(); // keep paths alive

            // Before start
            if let Some(start) = start_sec {
                let (p1_tmp, p1_path) = make_temp_with_ext(output_format)?;
                let status = Command::new("ffmpeg")
                    .args([
                        "-y",
                        "-i",
                        in_path.to_str().unwrap(),
                        "-to",
                        &start.to_string(),
                        "-c",
                        "copy",
                        "-f",
                        output_format,
                        &p1_path,
                    ])
                    .status()
                    .map_err(|e| format!("ffmpeg part1: {}", e))?;
                if status.success() {
                    parts.push(p1_path.clone());
                    keepers.push(p1_tmp); // keep file alive
                }
            }

            // After end
            if let Some(end) = end_sec {
                let (p2_tmp, p2_path) = make_temp_with_ext(output_format)?;
                let status = Command::new("ffmpeg")
                    .args([
                        "-y",
                        "-i",
                        in_path.to_str().unwrap(),
                        "-ss",
                        &end.to_string(),
                        "-c",
                        "copy",
                        "-f",
                        output_format,
                        &p2_path,
                    ])
                    .status()
                    .map_err(|e| format!("ffmpeg part2: {}", e))?;
                if status.success() {
                    parts.push(p2_path.clone());
                    keepers.push(p2_tmp); // keep file alive
                }
            }

            if parts.is_empty() {
                return Err("no parts to concatenate".into());
            }

            // Concat list file
            let concat_list = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
            let concat_path = concat_list.into_temp_path();
            {
                let mut f = fs::File::create(&concat_path)
                    .map_err(|e| format!("create concat file: {}", e))?;
                for p in &parts {
                    writeln!(f, "file '{}'", p).map_err(|e| format!("write concat file: {}", e))?;
                }
            }

            // Final concat
            let status = Command::new("ffmpeg")
                .args([
                    "-y",
                    "-f",
                    "concat",
                    "-safe",
                    "0",
                    "-i",
                    concat_path.to_str().unwrap(),
                    "-c",
                    "copy",
                    "-f",
                    output_format,
                    out_path.to_str().unwrap(),
                ])
                .status()
                .map_err(|e| format!("ffmpeg concat: {}", e))?;

            if !status.success() {
                return Err("ffmpeg failed during remove concat".into());
            }

            // keepers live until here → files exist during concat
            return fs::read(&out_path).map_err(|e| format!("read concat out: {}", e));
        }

        _ => Err("Invalid action (must be 'keep' or 'remove')".into()),
    }
}
