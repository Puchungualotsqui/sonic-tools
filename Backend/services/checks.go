package services

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"mime/multipart"
	"net/http"
)

const MaxUploadSize int64 = 50 << 20 // 50 MiB
const MaxFiles = 10

// Small cushion for multipart overhead (bound is enforced by MaxBytesReader)
const multipartOverhead int64 = 50 << 20 // 50 MiB

// CheckRequestConstraints caps the body, parses the multipart form,
// validates file count and total size, and returns the file headers.
func CheckRequestConstraints(w http.ResponseWriter, r *http.Request) ([]*multipart.FileHeader, error) {
	// Hard cap total request body (files + fields + multipart overhead)
	r.Body = http.MaxBytesReader(w, r.Body, MaxUploadSize+multipartOverhead)

	// Parse multipart. Keep a reasonable in-memory buffer; rest spills to temp files.
	if err := r.ParseMultipartForm(32 << 20); err != nil { // 32 MiB memory buffer
		// Distinguish common cases for nicer messages
		if errors.Is(err, http.ErrMissingFile) {
			return nil, fmt.Errorf("no files selected")
		}
		// MaxBytesReader violation surfaces here as a generic parse error
		return nil, fmt.Errorf("failed to parse form (too large or invalid): %w", err)
	}

	files := r.MultipartForm.File["files"]
	if len(files) == 0 {
		return nil, fmt.Errorf("no files selected")
	}
	if len(files) > MaxFiles {
		return nil, fmt.Errorf("too many files: max %d", MaxFiles)
	}

	// Validate combined size (independent of MaxBytesReader for clearer errors)
	var total int64
	for _, fh := range files {
		// Prefer header-reported size; if zero/unknown, measure by reading.
		if fh.Size > 0 {
			total += fh.Size
		} else {
			f, err := fh.Open()
			if err != nil {
				return nil, fmt.Errorf("failed to open %q: %w", fh.Filename, err)
			}
			// Discard to count size without buffering in memory.
			n, err := io.Copy(io.Discard, f)
			_ = f.Close()
			if err != nil {
				return nil, fmt.Errorf("failed to read %q: %w", fh.Filename, err)
			}
			total += n
		}
		if total > MaxUploadSize {
			return nil, fmt.Errorf("total upload exceeds %d MB", MaxUploadSize>>20)
		}
	}

	return files, nil
}

func ReadConfig(r *http.Request) (map[string]any, error) {
	settingsJSON := r.FormValue("settings")
	if settingsJSON == "" {
		// If settings are truly required, keep this as an error.
		// Otherwise, return an empty map:
		// return map[string]any{}, nil
		return nil, fmt.Errorf("empty settings JSON")
	}
	fmt.Println("settings JSON (raw):", settingsJSON)

	var settings map[string]any
	if err := json.Unmarshal([]byte(settingsJSON), &settings); err != nil {
		return nil, fmt.Errorf("failed to parse settings JSON: %w", err)
	}

	delete(settings, "cover") // if you always remove it here

	return settings, nil
}

func OrderFiles(files []*multipart.FileHeader, order []string) []*multipart.FileHeader {
	ordered := make([]*multipart.FileHeader, 0, len(order))
	index := make(map[string]*multipart.FileHeader, len(files))
	for _, f := range files {
		index[f.Filename] = f
	}
	for _, name := range order {
		if fh, ok := index[name]; ok {
			ordered = append(ordered, fh)
		}
	}
	return ordered
}

func ReadUploadedFiles(files []*multipart.FileHeader) ([][]byte, []string, error) {
	var fileData [][]byte
	var fileNames []string

	for _, fh := range files {
		f, err := fh.Open()
		if err != nil {
			return nil, nil, fmt.Errorf("open %q: %w", fh.Filename, err)
		}
		data, err := io.ReadAll(f)
		_ = f.Close()
		if err != nil {
			return nil, nil, fmt.Errorf("read %q: %w", fh.Filename, err)
		}
		fileData = append(fileData, data)
		fileNames = append(fileNames, fh.Filename)
	}

	return fileData, fileNames, nil
}
