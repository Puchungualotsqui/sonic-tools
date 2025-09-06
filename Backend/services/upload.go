package services

import (
	"encoding/json"
	"errors"
	"fmt"
	"mime/multipart"
	"net/http"
)

const MaxFreeTierUploadSize = 10 << 20
const MaxPremiumTierUploadSize = 50 << 20

func CheckRequestConstrictions(r *http.Request) error {
	size := r.ContentLength

	if size == -1 {
		return errors.New("could not determine request size")
	}

	if size > MaxPremiumTierUploadSize {
		return fmt.Errorf("request size: %d bytes. Max request size: %d bytes. Reduce file size",
			size, MaxPremiumTierUploadSize)
	}

	return nil
}

func ReadConfig(r *http.Request) (map[string]any, error) {
	settingsJSON := r.FormValue("settings")
	if settingsJSON == "" {
		return nil, fmt.Errorf("empty settings JSON")
	}

	fmt.Println("Raw settings JSON:", settingsJSON)

	var settings map[string]any
	if err := json.Unmarshal([]byte(settingsJSON), &settings); err != nil {
		return nil, fmt.Errorf("failed to parse settings JSON: %w", err)
	}

	fmt.Printf("Parsed settings")
	return settings, nil
}

func OrderFiles(files []*multipart.FileHeader, order []string) []*multipart.FileHeader {
	ordered := make([]*multipart.FileHeader, 0, len(order))
	for _, name := range order {
		for _, f := range files {
			if f.Filename == name {
				ordered = append(ordered, f)
				break
			}
		}
	}
	return ordered
}
