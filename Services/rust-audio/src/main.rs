use rust_audio::audio::convert_audio_server::ConvertAudioServer;
use rust_audio::audio::trim_audio_server::TrimAudioServer;
use rust_audio::services::convert::ConvertService;
use rust_audio::services::trim::TrimService;
use tonic::transport::Server;

use rust_audio::audio::compress_audio_server::CompressAudioServer;
use rust_audio::services::compress::CompressService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("Audio service running on {}", addr);

    Server::builder()
        .add_service(CompressAudioServer::new(CompressService::default()))
        .add_service(ConvertAudioServer::new(ConvertService::default()))
        .add_service(TrimAudioServer::new(TrimService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
