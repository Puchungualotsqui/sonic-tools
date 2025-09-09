use std::fs;
use std::process::Command;
use tempfile::{Builder, NamedTempFile};

enum CoverMode {
    None,
    Mp3Attached,  // ID3v2 attached picture
    Mp4CoverAtom, // MP4/M4A cover
}

struct MetaPlan {
    muxer: &'static str,
    extra_args: &'static [&'static str],
    cover_mode: CoverMode,
}

fn plan_for_meta(ext: &str) -> Result<MetaPlan, String> {
    match ext.to_ascii_lowercase().as_str() {
        "m4a" => Ok(MetaPlan {
            muxer: "mp4",
            extra_args: &["-movflags", "+faststart"],
            cover_mode: CoverMode::Mp4CoverAtom,
        }),
        "wma" => Ok(MetaPlan {
            muxer: "asf",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }), // text tags only
        "aac" => Ok(MetaPlan {
            muxer: "adts",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }), // no tagging in ADTS
        "mp3" => Ok(MetaPlan {
            muxer: "mp3",
            extra_args: &["-id3v2_version", "3", "-write_id3v1", "1"],
            cover_mode: CoverMode::Mp3Attached,
        }),
        "ogg" => Ok(MetaPlan {
            muxer: "ogg",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }),
        "opus" => Ok(MetaPlan {
            muxer: "ogg",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }),
        "wav" => Ok(MetaPlan {
            muxer: "wav",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }),
        "flac" => Ok(MetaPlan {
            muxer: "flac",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }),
        "aiff" | "aif" => Ok(MetaPlan {
            muxer: "aiff",
            extra_args: &[],
            cover_mode: CoverMode::None,
        }),
        other => Err(format!("Unsupported extension for metadata: {other}")),
    }
}

fn any_metadata_requested(
    title: &Option<String>,
    artist: &Option<String>,
    album: &Option<String>,
    year: &Option<String>,
    cover: &Option<Vec<u8>>,
) -> bool {
    title.as_deref().map_or(false, |s| !s.trim().is_empty())
        || artist.as_deref().map_or(false, |s| !s.trim().is_empty())
        || album.as_deref().map_or(false, |s| !s.trim().is_empty())
        || year.as_deref().map_or(false, |s| !s.trim().is_empty())
        || cover.is_some()
}

// helpers to build Vec<String> safely
fn push(args: &mut Vec<String>, items: &[&str]) {
    for s in items {
        args.push((*s).to_string());
    }
}
fn push_kv(args: &mut Vec<String>, k: &str, v: &str) {
    args.push(k.to_string());
    args.push(v.to_string());
}

pub fn write_metadata(
    input_bytes: Vec<u8>,
    ext: &str,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    year: Option<String>,
    cover_art: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    // ADTS AAC cannot carry tags/cover
    if ext.eq_ignore_ascii_case("aac")
        && any_metadata_requested(&title, &artist, &album, &year, &cover_art)
    {
        return Err(
            "Raw AAC (.aac/ADTS) does not support embedded metadata. Use .m4a instead.".into(),
        );
    }

    let plan = plan_for_meta(ext)?;

    // 1) temp input
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile: {e}"))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {e}"))?;
    let in_path = tmp_in.into_temp_path();

    // 2) temp output
    let tmp_out = Builder::new()
        .suffix(&format!(".{}", ext))
        .tempfile()
        .map_err(|e| format!("tmpfile: {e}"))?;
    let out_path = tmp_out.into_temp_path();

    // 3) build args
    let mut args: Vec<String> = Vec::with_capacity(32);
    push(
        &mut args,
        &["-y", "-hide_banner", "-loglevel", "error", "-i"],
    );
    args.push(in_path.to_str().ok_or("bad in_path")?.to_string());

    // Keep cover temp alive until after ffmpeg runs
    let mut cover_tmp: Option<NamedTempFile> = None;
    let mut used_cover = false;

    match (&plan.cover_mode, &cover_art) {
        (CoverMode::Mp3Attached, Some(bytes)) => {
            let ctmp = NamedTempFile::new().map_err(|e| format!("tmpfile cover: {e}"))?;
            fs::write(ctmp.path(), bytes).map_err(|e| format!("write cover: {e}"))?;
            push(&mut args, &["-i"]);
            args.push(ctmp.path().to_str().ok_or("bad cover path")?.to_string());
            push(
                &mut args,
                &[
                    "-map",
                    "0:a",
                    "-map",
                    "1:v",
                    "-c:a",
                    "copy",
                    "-c:v",
                    "mjpeg",
                    "-disposition:v",
                    "attached_pic",
                ],
            );
            cover_tmp = Some(ctmp);
            used_cover = true;
        }
        (CoverMode::Mp4CoverAtom, Some(bytes)) => {
            let ctmp = NamedTempFile::new().map_err(|e| format!("tmpfile cover: {e}"))?;
            fs::write(ctmp.path(), bytes).map_err(|e| format!("write cover: {e}"))?;
            push(&mut args, &["-i"]);
            args.push(ctmp.path().to_str().ok_or("bad cover path")?.to_string());
            push(
                &mut args,
                &["-map", "0:a", "-map", "1:v", "-c:a", "copy", "-c:v", "copy"],
            );
            cover_tmp = Some(ctmp);
            used_cover = true;
        }
        _ => {
            // no cover or unsupported: copy audio-only to drop any video/subs
            push(&mut args, &["-map", "0:a", "-c", "copy"]);
        }
    }

    // per-format extra flags
    for s in plan.extra_args {
        args.push(s.to_string());
    }

    // text metadata
    if let Some(t) = title.filter(|s| !s.trim().is_empty()) {
        push_kv(&mut args, "-metadata", &format!("title={}", t));
    }
    if let Some(a) = artist.filter(|s| !s.trim().is_empty()) {
        push_kv(&mut args, "-metadata", &format!("artist={}", a));
    }
    if let Some(al) = album.filter(|s| !s.trim().is_empty()) {
        push_kv(&mut args, "-metadata", &format!("album={}", al));
    }
    if let Some(y) = year.filter(|s| !s.trim().is_empty()) {
        push_kv(&mut args, "-metadata", &format!("date={}", y));
    }

    // muxer + output
    push(&mut args, &["-f", plan.muxer]);
    args.push(out_path.to_str().ok_or("bad out_path")?.to_string());

    // 4) run ffmpeg
    let output = Command::new("ffmpeg")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run ffmpeg: {e}"))?;

    drop(cover_tmp); // explicit, though it drops anyway here

    if !output.status.success() {
        let mut err = String::from_utf8_lossy(&output.stderr).to_string();
        if used_cover
            && ext.eq_ignore_ascii_case("m4a")
            && err.contains("could not find tag for codec")
        {
            err.push_str("\nHint: some cover formats are not allowed in MP4. Try JPEG or PNG.");
        }
        return Err(format!("ffmpeg failed: {}", err));
    }

    // 5) read result
    let bytes = fs::read(&out_path).map_err(|e| format!("read tmp out: {e}"))?;
    Ok(bytes)
}
