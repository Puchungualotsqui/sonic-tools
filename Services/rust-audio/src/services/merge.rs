// src/services/merge.rs
use crate::audio::{AudioResponse, MergeRequest, merge_audio_server::MergeAudio};
use crate::utils::merge::merge_sequential;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct MergeService {}

#[tonic::async_trait]
impl MergeAudio for MergeService {
    async fn merge(
        &self,
        request: Request<MergeRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        let req = request.into_inner();

        if req.file_data.is_empty() {
            return Err(Status::invalid_argument("no files provided"));
        }
        if req.filenames.len() != req.file_data.len() {
            return Err(Status::invalid_argument(
                "filenames and file_data length mismatch",
            ));
        }

        // Keep original order based on `filenames` array (already aligned by index)
        // Pair them so utils can keep order stable.
        let mut inputs: Vec<(String, Vec<u8>)> = Vec::with_capacity(req.file_data.len());
        for (i, bytes) in req.file_data.into_iter().enumerate() {
            let name = req
                .filenames
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("in_{}.dat", i));
            inputs.push((name, bytes));
        }

        let out_fmt = req.output_format.to_lowercase();
        if out_fmt.is_empty() {
            return Err(Status::invalid_argument(
                "output_format required (e.g., mp3, wav, flac, m4a)",
            ));
        }

        match merge_sequential(inputs, &out_fmt) {
            Ok(bytes) => Ok(Response::new(AudioResponse {
                file_data: bytes,
                format: out_fmt.clone(),
                filename: format!("merged.{}", out_fmt),
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }
}
