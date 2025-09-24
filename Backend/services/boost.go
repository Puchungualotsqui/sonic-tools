package services

import (
	"context"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc"
)

func NewBoostClient() (pb.BoostAudioClient, *grpc.ClientConn, error) {
	conn, err := NewClient()
	if err != nil {
		return nil, nil, err
	}
	client := pb.NewBoostAudioClient(conn)
	return client, conn, nil
}

func BoostManual(files [][]byte, filenames []string, gain int32) (*pb.AudioResponse, error) {
	client, conn, err := NewBoostClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.BoostManualRequest{
		FileData:  files,
		Filenames: filenames,
		Gain:      gain,
	}

	return client.BoostManual(ctx, req)
}

func BoostNormalize(files [][]byte, filenames []string) (*pb.AudioResponse, error) {
	client, conn, err := NewBoostClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.BoostNormalizeRequest{
		FileData:  files,
		Filenames: filenames,
	}

	return client.BoostNormalize(ctx, req)
}
