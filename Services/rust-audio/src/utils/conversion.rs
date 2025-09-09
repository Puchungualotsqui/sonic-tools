use std::fs;
use std::process::Command;
use tempfile::{Builder, NamedTempFile};

/// Describes how to encode/mux for a requested "output_format" string.
struct EncodePlan {
    /// File extension to use for the output file (e.g. "m4a", "aac", "wav").
    out_ext: &'static str,
    /// ffmpeg muxer name to pass to `-f` (e.g. "mp4", "adts", "wav", "flac").
    muxer: &'static str,
    /// Extra args to add before `-f` (codec, flags, etc.)
    pre_f_args: &'static [&'static str],
    /// Whether a kilobit bitrate (-b:a NNk) makes sense (e.g. not for lossless).
    supports_bitrate: bool,
}

fn plan_for(format: &str) -> Result<EncodePlan, String> {
    match format.to_ascii_lowercase().as_str() {
        // Lossy
        "mp3" => Ok(EncodePlan {
            out_ext: "mp3",
            muxer: "mp3",
            pre_f_args: &["-c:a", "libmp3lame"],
            supports_bitrate: true,
        }),
        "ogg" => Ok(EncodePlan {
            out_ext: "ogg",
            muxer: "ogg",
            pre_f_args: &["-c:a", "libvorbis"],
            supports_bitrate: true,
        }),
        "opus" => Ok(EncodePlan {
            out_ext: "opus",
            muxer: "ogg",
            pre_f_args: &["-c:a", "libopus"],
            supports_bitrate: true,
        }),
        "aac" => Ok(EncodePlan {
            out_ext: "aac",
            muxer: "adts",
            pre_f_args: &["-c:a", "aac"],
            supports_bitrate: true,
        }),
        "m4a" => Ok(EncodePlan {
            out_ext: "m4a",
            muxer: "mp4",
            pre_f_args: &["-c:a", "aac", "-movflags", "+faststart"],
            supports_bitrate: true,
        }),
        "wma" => Ok(EncodePlan {
            out_ext: "wma",
            muxer: "asf",
            // Many players expect 44.1kHz stereo; set it for compatibility.
            pre_f_args: &["-c:a", "wmav2", "-ar", "44100", "-ac", "2"],
            supports_bitrate: true,
        }),

        // Lossless
        "wav" => Ok(EncodePlan {
            out_ext: "wav",
            muxer: "wav",
            pre_f_args: &[],
            supports_bitrate: false,
        }),
        "flac" => Ok(EncodePlan {
            out_ext: "flac",
            muxer: "flac",
            pre_f_args: &[],
            supports_bitrate: false,
        }),
        "aiff" | "aif" => Ok(EncodePlan {
            out_ext: "aiff",
            muxer: "aiff",
            pre_f_args: &[],
            supports_bitrate: false,
        }),

        other => Err(format!("Unsupported output format: {}", other)),
    }
}

/// Optionally pass an input extension so ffmpeg can sniff more reliably for some files.
/// If you don't have it, pass `None`.
pub fn convert_file(
    input_bytes: Vec<u8>,
    output_format: &str,
    bitrate: i32,
    input_ext: Option<&str>,
) -> Result<Vec<u8>, String> {
    let plan = plan_for(output_format)?;

    // Write input to a temp file — include the original extension if we know it.
    let tmp_in = if let Some(ext) = input_ext {
        Builder::new()
            .suffix(&format!(".{}", ext.trim_start_matches('.')))
            .tempfile()
            .map_err(|e| format!("tmpfile (in): {}", e))?
    } else {
        NamedTempFile::new().map_err(|e| format!("tmpfile (in): {}", e))?
    };
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    // Prepare output temp file with the planned extension.
    let tmp_out = Builder::new()
        .suffix(&format!(".{}", plan.out_ext))
        .tempfile()
        .map_err(|e| format!("tmpfile (out): {}", e))?;
    let out_path = tmp_out.into_temp_path();

    // Build ffmpeg command
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-hide_banner", "-loglevel", "error"]);
    cmd.args(["-i", in_path.to_str().ok_or("bad in_path")?]);

    // Bitrate only where it makes sense and is provided
    if bitrate > 0 && plan.supports_bitrate {
        cmd.args(["-b:a", &format!("{}k", bitrate)]);
    }

    // Codec/flags that should come before -f
    if !plan.pre_f_args.is_empty() {
        cmd.args(plan.pre_f_args);
    }

    // Set muxer explicitly based on plan
    cmd.args(["-f", plan.muxer, out_path.to_str().ok_or("bad out_path")?]);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run ffmpeg: {}", e))?;
    if !status.success() {
        return Err("ffmpeg failed".into());
    }

    let bytes = fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))?;
    Ok(bytes)
}
