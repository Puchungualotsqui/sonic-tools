use std::fs;
use std::process::Command;
use tempfile::{Builder, NamedTempFile};

pub fn write_metadata(
    input_bytes: Vec<u8>,
    ext: &str,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    year: Option<String>,
    cover_art: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    let tmp_out = Builder::new()
        .suffix(&format!(".{}", ext))
        .tempfile()
        .map_err(|e| format!("tmpfile: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    let mut args = vec![
        "-y".into(),
        "-i".into(),
        in_path.to_str().ok_or("bad in_path")?.into(),
    ];

    // Optional cover art
    let cover_tmp;
    if let Some(cover) = cover_art {
        cover_tmp = NamedTempFile::new().map_err(|e| format!("tmpfile: {}", e))?;
        fs::write(cover_tmp.path(), &cover).map_err(|e| format!("write cover: {}", e))?;
        args.push("-i".into());
        args.push(cover_tmp.path().to_str().ok_or("bad cover path")?.into());
        args.extend(vec![
            "-map".into(),
            "0".into(),
            "-map".into(),
            "1".into(),
            "-c".into(),
            "copy".into(),
            "-id3v2_version".into(),
            "3".into(),
        ]);
    }

    // Metadata fields
    if let Some(t) = title {
        if !t.trim().is_empty() {
            args.push("-metadata".into());
            args.push(format!("title={}", t));
        }
    }

    if let Some(a) = artist {
        if !a.trim().is_empty() {
            args.push("-metadata".into());
            args.push(format!("artist={}", a));
        }
    }

    if let Some(al) = album {
        if !al.trim().is_empty() {
            args.push("-metadata".into());
            args.push(format!("album={}", al));
        }
    }

    if let Some(y) = year {
        if !y.trim().is_empty() {
            args.push("-metadata".into());
            args.push(format!("date={}", y));
        }
    }

    args.push(out_path.to_str().ok_or("bad out_path")?.into());

    let status = Command::new("ffmpeg")
        .args(&args)
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;

    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    let bytes = fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))?;

    Ok(bytes)
}
