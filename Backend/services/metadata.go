package services

import (
	"context"
	"io"
	"net/http"
	"time"

	pb "backend/proto/backend/proto"

	"google.golang.org/grpc"
)

func NewMetadataClient() (pb.MetadataAudioClient, *grpc.ClientConn, error) {
	conn, err := NewClient()
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
	file, _, err := r.FormFile("cover")
	if err != nil {
		// No cover uploaded
		return nil, nil
	}
	defer file.Close()

	data, err := io.ReadAll(file)
	if err != nil {
		return nil, err
	}

	return data, nil
}
