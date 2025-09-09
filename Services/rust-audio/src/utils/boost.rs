use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

struct EncodePlan {
    muxer: &'static str,
    pre_f_args: &'static [&'static str], // placed before -f (codec/flags)
}

fn plan_for(fmt: &str) -> Result<EncodePlan, String> {
    match fmt.to_ascii_lowercase().as_str() {
        // Lossy
        "mp3" => Ok(EncodePlan {
            muxer: "mp3",
            pre_f_args: &["-c:a", "libmp3lame"],
        }),
        "ogg" => Ok(EncodePlan {
            muxer: "ogg",
            pre_f_args: &["-c:a", "libvorbis"],
        }),
        "opus" => Ok(EncodePlan {
            muxer: "ogg",
            pre_f_args: &["-c:a", "libopus"],
        }),
        "aac" => Ok(EncodePlan {
            muxer: "adts",
            pre_f_args: &["-c:a", "aac"],
        }),
        "m4a" => Ok(EncodePlan {
            muxer: "mp4",
            pre_f_args: &["-c:a", "aac", "-movflags", "+faststart"],
        }),
        "wma" => Ok(EncodePlan {
            muxer: "asf",
            pre_f_args: &["-c:a", "wmav2", "-ar", "44100", "-ac", "2"],
        }),

        // Lossless / PCM
        "wav" => Ok(EncodePlan {
            muxer: "wav",
            pre_f_args: &["-c:a", "pcm_s16le"],
        }),
        "flac" => Ok(EncodePlan {
            muxer: "flac",
            pre_f_args: &["-c:a", "flac"],
        }),
        "aiff" | "aif" => Ok(EncodePlan {
            muxer: "aiff",
            pre_f_args: &["-c:a", "pcm_s16be"],
        }),

        other => Err(format!("Unsupported output format: {}", other)),
    }
}

fn run_ffmpeg_filter(
    input_bytes: Vec<u8>,
    output_format: &str,
    afilter: &str,
) -> Result<Vec<u8>, String> {
    // temp in
    let tmp_in = NamedTempFile::new().map_err(|e| format!("tmpfile in: {}", e))?;
    fs::write(tmp_in.path(), &input_bytes).map_err(|e| format!("write tmp in: {}", e))?;
    let in_path = tmp_in.into_temp_path();

    // temp out
    let tmp_out = NamedTempFile::new().map_err(|e| format!("tmpfile out: {}", e))?;
    let out_path = tmp_out.into_temp_path();

    // plan
    let plan = plan_for(output_format)?;

    // build
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-hide_banner", "-loglevel", "error"]);
    cmd.args(["-i", in_path.to_str().ok_or("bad in_path")?]);
    cmd.args(["-af", afilter]);
    if !plan.pre_f_args.is_empty() {
        cmd.args(plan.pre_f_args);
    }
    cmd.args(["-f", plan.muxer, out_path.to_str().ok_or("bad out_path")?]);

    // run
    let output = cmd.output().map_err(|e| format!("ffmpeg exec: {}", e))?;
    if !output.status.success() {
        return Err(format!(
            "ffmpeg failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // read
    fs::read(&out_path).map_err(|e| format!("read tmp out: {}", e))
}

pub fn boost_file(input_bytes: Vec<u8>, output_format: &str, gain: i32) -> Result<Vec<u8>, String> {
    // Positive gain boosts, negative attenuates (e.g., -3 dB)
    let afilter = format!("volume={}dB", gain);
    run_ffmpeg_filter(input_bytes, output_format, &afilter)
}

pub fn normalize_file(input_bytes: Vec<u8>, output_format: &str) -> Result<Vec<u8>, String> {
    // One-pass EBU R128; good defaults for music/podcasts.
    // If you ever want the more precise two-pass, we can add it later.
    let afilter = "loudnorm=I=-16:TP=-1.5:LRA=11";
    run_ffmpeg_filter(input_bytes, output_format, afilter)
}
