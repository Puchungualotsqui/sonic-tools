use crate::audio::{
    AudioResponse, CompressPercentageRequest, CompressQualityRequest, CompressSizeRequest,
    compress_audio_server::CompressAudio,
};
use crate::utils::compress::compress_file;
use crate::utils::ffmpeg::{probe_bitrate, probe_duration};
use crate::utils::zip::make_zip;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct CompressService {}

#[tonic::async_trait]
impl CompressAudio for CompressService {
    async fn compress_percentage(
        &self,
        request: Request<CompressPercentageRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        println!("Starting compression by percentage");
        let req = request.into_inner();
        let mut outputs = Vec::new();

        for (i, data) in req.file_data.into_iter().enumerate() {
            let filename = req.filenames.get(i).cloned().unwrap_or("output".into());
            println!("filename gotten");
            let ext = filename.split('.').last().unwrap_or("mp3");
            println!("extension goten");

            let original_bitrate = match probe_bitrate(&data, ext) {
                Ok(b) => b,
                Err(e) => return Err(Status::internal(e)),
            };

            println!("original bitrate gotten");
            let mut target_bitrate =
                ((original_bitrate as f32) * (req.percentage as f32 / 100.0)) as i32 / 1000;
            println!("first target bitrate set");

            if target_bitrate < 32 {
                target_bitrate = 32
            }
            println!("final target bitrate set");

            let out_name = format!(
                "{}.{}",
                filename.trim_end_matches(&format!(".{}", ext)),
                ext
            );
            println!("compressin file");
            match compress_file(data, ext, Some(target_bitrate)) {
                Ok(bytes) => outputs.push((out_name, bytes)),
                Err(e) => return Err(Status::internal(e)),
            }
        }

        if outputs.len() == 1 {
            let (filename, bytes) = outputs.into_iter().next().unwrap();
            let ext = filename.split('.').last().unwrap_or("mp3");
            Ok(Response::new(AudioResponse {
                file_data: bytes,
                format: ext.to_string(),
                filename: filename,
            }))
        } else {
            match make_zip(outputs) {
                Ok(zip_bytes) => Ok(Response::new(AudioResponse {
                    file_data: zip_bytes,
                    format: "zip".to_string(),
                    filename: "sonic-tools.zip".to_string(),
                })),
                Err(e) => Err(Status::internal(e)),
            }
        }
    }

    async fn compress_size(
        &self,
        request: Request<CompressSizeRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        println!("compress started");
        let req = request.into_inner();
        let mut outputs = Vec::new();

        for (i, data) in req.file_data.into_iter().enumerate() {
            println!("loop starting");
            let filename = req.filenames.get(i).cloned().unwrap_or("output".into());
            let ext = filename.split('.').last().unwrap_or("mp3");

            // 1. Probe duration
            let duration = match probe_duration(&data) {
                Ok(d) => d,
                Err(e) => return Err(Status::internal(e)),
            };

            // 2. Calculate target bitrate
            let target_size_bytes = (req.size as u64) * 1024 * 1024;
            let target_bitrate = ((target_size_bytes as f32 * 8.0) / duration) as i32 / 1000;

            // enforce min bitrate
            let target_bitrate = target_bitrate.max(32);

            let out_name = format!(
                "{}_compressed.{}",
                filename.trim_end_matches(&format!(".{}", ext)),
                ext
            );

            println!("Target bitrate calc: {} kbps", target_bitrate);

            match compress_file(data, ext, Some(target_bitrate)) {
                Ok(bytes) => outputs.push((out_name, bytes)),
                Err(e) => return Err(Status::internal(e)),
            }

            println!("one looped finished");
        }

        println!("loop finished");
        if outputs.len() == 1 {
            let (filename, bytes) = outputs.into_iter().next().unwrap();
            let ext = filename.split('.').last().unwrap_or("mp3");
            Ok(Response::new(AudioResponse {
                file_data: bytes,
                format: ext.to_string(),
                filename,
            }))
        } else {
            match make_zip(outputs) {
                Ok(zip_bytes) => Ok(Response::new(AudioResponse {
                    file_data: zip_bytes,
                    format: "zip".to_string(),
                    filename: "sonic-tools.zip".to_string(),
                })),
                Err(e) => Err(Status::internal(e)),
            }
        }
    }

    async fn compress_quality(
        &self,
        request: Request<CompressQualityRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        let req = request.into_inner();
        let mut outputs = Vec::new();

        println!("compress_quality loop starting");
        for (i, data) in req.file_data.into_iter().enumerate() {
            let filename = req.filenames.get(i).cloned().unwrap_or("output".into());
            let ext = filename.split('.').last().unwrap_or("mp3");
            let bitrate = match req.quality.as_str() {
                "low" => Some(64),
                "medium" => Some(128),
                "high" => Some(256),
                _ => Some(128),
            };

            match compress_file(data, ext, bitrate) {
                Ok(bytes) => outputs.push((filename, bytes)),
                Err(e) => return Err(Status::internal(e)),
            }
            println!("one looped finished");
        }
        print!("loop finished");

        if outputs.len() == 1 {
            let (filename, bytes) = outputs.into_iter().next().unwrap();
            let ext = filename.split('.').last().unwrap_or("mp3");
            Ok(Response::new(AudioResponse {
                file_data: bytes,
                format: ext.to_string(),
                filename: filename,
            }))
        } else {
            println!("starting to make zip");
            match make_zip(outputs) {
                Ok(zip_bytes) => Ok(Response::new(AudioResponse {
                    file_data: zip_bytes,
                    format: "zip".to_string(),
                    filename: "sonic-tools.zip".to_string(),
                })),
                Err(e) => Err(Status::internal(e)),
            }
        }
    }
}
