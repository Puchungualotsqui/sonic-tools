package handlers

import (
	audio "backend/proto/backend/proto"
	"backend/services"
	"backend/utils"
	"fmt"
	"log"
	"net/http"
)

func UploadHandler(w http.ResponseWriter, r *http.Request) {
	// Check uploaded files constrains
	var err error
	err = services.CheckRequestConstrictions(r)
	if err != nil {
		log.Println("Upload error:", err)
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Read settings JSON if provided
	settings, err := services.ReadConfig(r)
	if err != nil {
		log.Println("Read config file error:", err)
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	fileOrder := utils.GetArrayString(settings["fileOrder"])
	filesLength := len(r.MultipartForm.File["files"])
	fileOrderLength := len(fileOrder)
	if filesLength < 1 {
		log.Printf("invalid files length: %d", filesLength)
		http.Error(w, "Not files selected", http.StatusBadRequest)
		return
	} else if fileOrderLength != filesLength {
		log.Printf("fileOrder length: %d, files length: %d",
			fileOrderLength, filesLength)
		http.Error(w, "Upload error", http.StatusBadRequest)
		return
	}

	files := services.OrderFiles(r.MultipartForm.File["files"], fileOrder)

	fileData, fileNames, err := services.ReadUploadedFiles(files)
	if err != nil {
		http.Error(w, "Failed to read uploaded files: "+err.Error(), http.StatusInternalServerError)
		return
	}
	tool := settings["tool"]

	var resp *audio.AudioResponse

	switch tool {
	case "compress":
		method := settings["method"]
		switch method {
		case "mb":
			targetSize := utils.TryGetValue(settings, "mb", "1")
			resp, err = services.CompressQuality(fileData, fileNames, targetSize)
			if err != nil {
				http.Error(w, "Compression failed: "+err.Error(), http.StatusInternalServerError)
				return
			}
		case "percentage":
			targetPercentage := utils.TryGetValue(settings, "percentage", int32(1))
			resp, err = services.CompressPercentage(fileData, fileNames, targetPercentage)
			if err != nil {
				http.Error(w, "Compression failed: "+err.Error(), http.StatusInternalServerError)
				return
			}
		case "quality":
			quality := utils.TryGetValue(settings, "quality", "medium")
			resp, err = services.CompressQuality(fileData, fileNames, quality)
			fmt.Println("compression finished")
			if err != nil {
				http.Error(w, "Compression failed: "+err.Error(), http.StatusInternalServerError)
				return
			}
		}
	}

	// Set headers so browser downloads the file
	w.Header().Set("Content-Disposition", "attachment; filename="+resp.Filename)
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Write(resp.FileData)
}
