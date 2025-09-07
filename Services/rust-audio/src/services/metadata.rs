use crate::audio::{AudioResponse, MetadataRequest, metadata_audio_server::MetadataAudio};
use crate::utils::metadata::write_metadata;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MetadataService {}

#[tonic::async_trait]
impl MetadataAudio for MetadataService {
    async fn metadata(
        &self,
        request: Request<MetadataRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        println!("Starting metadata write");

        let req = request.into_inner();
        let filename = req.filename;
        let ext = filename.split('.').last().unwrap_or("mp3");

        match write_metadata(
            req.file_data,
            ext,
            req.title,
            req.artist,
            req.album,
            req.year,
            req.cover_art,
        ) {
            Ok(bytes) => Ok(Response::new(AudioResponse {
                file_data: bytes,
                format: ext.to_string(),
                filename,
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }
}
