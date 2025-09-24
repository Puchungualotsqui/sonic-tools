package services

import (
	"context"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc"
)

func NewTrimClient() (pb.TrimAudioClient, *grpc.ClientConn, error) {
	conn, err := NewClient()
	if err != nil {
		return nil, nil, err
	}
	client := pb.NewTrimAudioClient(conn)
	return client, conn, nil
}

func Trim(files []byte, filename string, startMs, endMs int32, action string) (*pb.AudioResponse, error) {
	client, conn, err := NewTrimClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.TrimRequest{
		FileData: files,
		Filename: filename,
		StartS:   &startMs,
		EndS:     &endMs,
		Action:   action,
	}

	return client.Trim(ctx, req)
}
