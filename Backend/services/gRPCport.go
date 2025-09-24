package services

import (
	"log"
	"os"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func grpcAddr() string {
	addr := os.Getenv("RUST_AUDIO_ADDR")
	if addr == "" {
		addr = "rust-audio:50051" // fallback default
	}
	log.Printf("Dialing gRPC at %s", addr)
	return addr
}

func NewClient() (*grpc.ClientConn, error) {
	conn, err := grpc.NewClient(
		grpcAddr(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithDefaultCallOptions(
			grpc.MaxCallRecvMsgSize(100*1024*1024), // allow up to 100 MB
			grpc.MaxCallSendMsgSize(100*1024*1024), // allow up to 100 MB
		),
	)

	return conn, err
}
