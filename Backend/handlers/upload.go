package handlers

import (
	"backend/services"
	"backend/utils"
	"io"
	"log"
	"net/http"
)

func UploadHandler(w http.ResponseWriter, r *http.Request) {
	// Check uploaded files constrains
	err := services.CheckRequestConstrictions(r)
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

	// For testing, just return the first file as a download
	fileHeader := files[0]
	file, err := fileHeader.Open()
	if err != nil {
		http.Error(w, "Failed to open file", http.StatusInternalServerError)
		return
	}
	defer file.Close()

	// Set headers so browser downloads the file
	w.Header().Set("Content-Disposition", "attachment; filename="+fileHeader.Filename)
	w.Header().Set("Content-Type", "application/octet-stream")

	_, err = io.Copy(w, file)
	if err != nil {
		http.Error(w, "Failed to send file", http.StatusInternalServerError)
		return
	}
}
