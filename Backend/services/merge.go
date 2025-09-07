package services

import (
	"context"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc/credentials/insecure"

	"google.golang.org/grpc"
)

func NewMergeClient() (pb.MergeAudioClient, *grpc.ClientConn, error) {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, nil, err
	}
	client := pb.NewMergeAudioClient(conn)
	return client, conn, nil
}

func Merge(files [][]byte, filenames []string, outputFormat string) (*pb.AudioResponse, error) {
	client, conn, err := NewMergeClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.MergeRequest{
		FileData:     files,
		Filenames:    filenames,
		OutputFormat: outputFormat,
	}

	return client.Merge(ctx, req)
}
