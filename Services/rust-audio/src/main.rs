use rust_audio::audio::boost_audio_server::BoostAudioServer;
use rust_audio::audio::convert_audio_server::ConvertAudioServer;
use rust_audio::audio::merge_audio_server::MergeAudioServer;
use rust_audio::audio::metadata_audio_server::MetadataAudioServer;
use rust_audio::audio::trim_audio_server::TrimAudioServer;
use rust_audio::services::boost::BoostService;
use rust_audio::services::convert::ConvertService;
use rust_audio::services::merge::MergeService;
use rust_audio::services::metadata::MetadataService;
use rust_audio::services::trim::TrimService;
use tonic::transport::Server;

use rust_audio::audio::compress_audio_server::CompressAudioServer;
use rust_audio::services::compress::CompressService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    println!("Audio service running on {}", addr);

    Server::builder()
        .add_service(
            CompressAudioServer::new(CompressService::default())
                .max_decoding_message_size(100 * 1024 * 1024)
                .max_encoding_message_size(100 * 1024 * 1024),
        )
        .add_service(
            ConvertAudioServer::new(ConvertService::default())
                .max_decoding_message_size(100 * 1024 * 1024)
                .max_encoding_message_size(100 * 1024 * 1024),
        )
        .add_service(
            TrimAudioServer::new(TrimService::default())
                .max_decoding_message_size(100 * 1024 * 1024)
                .max_encoding_message_size(100 * 1024 * 1024),
        )
        .add_service(
            MergeAudioServer::new(MergeService::default())
                .max_decoding_message_size(100 * 1024 * 1024)
                .max_encoding_message_size(100 * 1024 * 1024),
        )
        .add_service(
            MetadataAudioServer::new(MetadataService::default())
                .max_decoding_message_size(100 * 1024 * 1024)
                .max_encoding_message_size(100 * 1024 * 1024),
        )
        .add_service(
            BoostAudioServer::new(BoostService::default())
                .max_decoding_message_size(100 * 1024 * 1024)
                .max_encoding_message_size(100 * 1024 * 1024),
        )
        .serve(addr)
        .await?;

    Ok(())
}
