use crate::audio::{AudioResponse, ConvertRequest, convert_audio_server::ConvertAudio};
use crate::utils::conversion::convert_file;
use crate::utils::zip::make_zip;
use std::path::Path;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct ConvertService {}

#[tonic::async_trait]
impl ConvertAudio for ConvertService {
    async fn convert(
        &self,
        request: Request<ConvertRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        println!("Starting convert service");
        let req = request.into_inner();
        let mut outputs = Vec::new();

        for (i, data) in req.file_data.into_iter().enumerate() {
            println!("Loop {}: got {} bytes", i, data.len());

            let filename = req.filenames.get(i).cloned().unwrap_or("output".into());
            let ext = req.output_format.as_str();

            let stem = Path::new(&filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            println!("Original filename: {}", filename);

            let out_name = format!("{}.{}", stem, ext);

            match convert_file(data, ext, req.bitrate) {
                Ok(bytes) => outputs.push((out_name, bytes)),
                Err(e) => return Err(Status::internal(e)),
            }
        }

        println!("Total outputs: {}", outputs.len());
        if outputs.len() == 1 {
            let (filename, bytes) = outputs.into_iter().next().unwrap();
            Ok(Response::new(AudioResponse {
                file_data: bytes,
                format: req.output_format,
                filename,
            }))
        } else {
            println!("Building zip with {} files", outputs.len());
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
