package services

import (
	"context"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc/credentials/insecure"

	"google.golang.org/grpc"
)

func NewConvertClient() (pb.ConvertAudioClient, *grpc.ClientConn, error) {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, nil, err
	}
	client := pb.NewConvertAudioClient(conn)
	return client, conn, nil
}

func Convert(files [][]byte, filenames []string, outputFormat string, bitrate int32) (*pb.AudioResponse, error) {
	client, conn, err := NewConvertClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.ConvertRequest{
		FileData:     files,
		OutputFormat: outputFormat,
		Bitrate:      bitrate,
	}

	return client.Convert(ctx, req)
}
