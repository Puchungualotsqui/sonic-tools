package services

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc/credentials/insecure"

	"google.golang.org/grpc"
)

func NewMetadataClient() (pb.MetadataAudioClient, *grpc.ClientConn, error) {
	conn, err := grpc.NewClient("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, nil, err
	}
	client := pb.NewMetadataAudioClient(conn)
	return client, conn, nil
}

func Metadata(file []byte, filename string, title, artist, album, year string, coverArt []byte) (*pb.AudioResponse, error) {
	client, conn, err := NewMetadataClient()
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	req := &pb.MetadataRequest{
		FileData: file,
		Filename: filename,
		Title:    &title,
		Artist:   &artist,
		Album:    &album,
		Year:     &year,
		CoverArt: coverArt,
	}

	return client.Metadata(ctx, req)
}

func GetCover(r *http.Request) ([]byte, error) {
	coverFiles := r.MultipartForm.File["cover"]
	if len(coverFiles) == 0 {
		return nil, nil // no cover uploaded
	}

	f, err := coverFiles[0].Open()
	if err != nil {
		return nil, fmt.Errorf("open cover file: %w", err)
	}
	defer f.Close()

	coverBytes, err := io.ReadAll(f)
	if err != nil {
		return nil, fmt.Errorf("read cover file: %w", err)
	}

	return coverBytes, nil
}
