use crate::audio::{AudioResponse, TrimRequest, trim_audio_server::TrimAudio};
use crate::utils::trim::trim_file;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct TrimService {}

#[tonic::async_trait]
impl TrimAudio for TrimService {
    async fn trim(&self, request: Request<TrimRequest>) -> Result<Response<AudioResponse>, Status> {
        println!("Trim request received");

        let req = request.into_inner();

        let filename = req.filename.clone(); // ideally rename to filename in proto
        let ext = filename.split('.').last().unwrap_or("mp3");

        // Default action
        let action = if req.action.is_empty() {
            "keep".to_string()
        } else {
            req.action
        };

        // Validate ranges
        if action == "keep" {
            if let (Some(start), Some(end)) = (req.start_s, req.end_s) {
                if start >= end {
                    return Err(Status::invalid_argument("start must be < end for keep"));
                }
            }
        }

        // Run ffmpeg trim
        let trimmed = match trim_file(req.file_data, ext, req.start_s, req.end_s, &action) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Trim error: {}", e);
                return Err(Status::internal(format!("Trim failed: {}", e)));
            }
        };

        Ok(Response::new(AudioResponse {
            file_data: trimmed,
            format: ext.to_string(),
            filename,
        }))
    }
}
