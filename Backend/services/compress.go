package services

import (
	"context"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc/credentials/insecure"

	"google.golang.org/grpc"
)

func NewCompressorClient() (pb.CompressAudioClient, *grpc.ClientConn, error) {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, nil, err
	}
	client := pb.NewCompressAudioClient(conn)
	return client, conn, nil
}

func CompressPercentage(files [][]byte, filenames []string, percentage int32) (*pb.AudioResponse, error) {
	client, conn, err := NewCompressorClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.CompressPercentageRequest{
		FileData:   files,
		Filenames:  filenames,
		Percentage: percentage,
	}

	return client.CompressPercentage(ctx, req)
}

func CompressSize(files [][]byte, filenames []string, size int32) (*pb.AudioResponse, error) {
	client, conn, err := NewCompressorClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.CompressSizeRequest{
		FileData:  files,
		Filenames: filenames,
		Size:      size,
	}

	return client.CompressSize(ctx, req)
}

func CompressQuality(files [][]byte, filenames []string, quality string) (*pb.AudioResponse, error) {
	client, conn, err := NewCompressorClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.CompressQualityRequest{
		FileData:  files,
		Filenames: filenames,
		Quality:   quality,
	}

	return client.CompressQuality(ctx, req)
}
