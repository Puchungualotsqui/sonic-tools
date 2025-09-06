use tonic::{transport::Server, Request, Response, Status};

pub mod audio {
    tonic::include_proto!("audio");
}

use audio::audio_processor_server::{AudioProcessor, AudioProcessorServer};
use audio::{TrimRequest, AudioResponse};

#[derive(Debug, Default)]
pub struct MyAudioProcessor {}

#[tonic::async_trait]
impl AudioProcessor for MyAudioProcessor {
    async fn trim(
        &self,
        request: Request<TrimRequest>,
    ) -> Result<Response<AudioResponse>, Status> {
        let req = request.into_inner();

        println!("Received trim request: {} bytes, start={}ms end={}ms",
            req.file_data.len(), req.start_ms, req.end_ms);

        // TODO: Replace with actual audio trimming.
        // For now, just return the same bytes back.
        let processed = req.file_data;

        let reply = AudioResponse {
            file_data: processed,
            format: "wav".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let processor = MyAudioProcessor::default();

    println!("Rust AudioProcessor running on {}", addr);

    Server::builder()
        .add_service(AudioProcessorServer::new(processor))
        .serve(addr)
        .await?;

    Ok(())
}
