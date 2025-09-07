use tonic::transport::Server;

use rust_audio::audio::compress_audio_server::CompressAudioServer;
use rust_audio::services::compress::CompressService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("Audio service running on {}", addr);

    Server::builder()
        .add_service(CompressAudioServer::new(CompressService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
