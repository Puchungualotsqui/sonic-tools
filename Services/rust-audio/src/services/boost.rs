use crate::audio::{
    AudioResponse, BoostManualRequest, BoostNormalizeRequest, boost_audio_server::BoostAudio,
};
use crate::utils::boost::{boost_file, normalize_file};
use crate::utils::zip::make_zip;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct BoostService {}

#[tonic::async_trait]
impl BoostAudio for BoostService {
    async fn boost_manual(
        &self,
        request: Request<BoostManualRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        let req = request.into_inner();
        let mut outputs = Vec::new();

        for (i, data) in req.file_data.into_iter().enumerate() {
            let filename = req.filenames.get(i).cloned().unwrap_or("output".into());
            let ext = filename.split('.').last().unwrap_or("mp3");

            match boost_file(data, ext, req.gain) {
                Ok(bytes) => outputs.push((filename, bytes)),
                Err(e) => return Err(Status::internal(e)),
            }
        }

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

    async fn boost_normalize(
        &self,
        request: Request<BoostNormalizeRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        let req = request.into_inner();
        let mut outputs = Vec::new();

        for (i, data) in req.file_data.into_iter().enumerate() {
            let filename = req.filenames.get(i).cloned().unwrap_or("output".into());
            let ext = filename.split('.').last().unwrap_or("mp3");

            match normalize_file(data, ext) {
                Ok(bytes) => outputs.push((filename, bytes)),
                Err(e) => return Err(Status::internal(e)),
            }
        }

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
}
