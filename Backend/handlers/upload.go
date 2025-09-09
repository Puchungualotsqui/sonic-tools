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
	// Validate & get file headers (also parses form)
	files, err := services.CheckRequestConstraints(w, r)
	if err != nil {
		log.Println("Upload validation error:", err)
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	// Read settings JSON
	settings, err := services.ReadConfig(r)
	if err != nil {
		log.Println("Read config file error:", err)
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	fileOrder := utils.GetArrayString(settings["fileOrder"])
	if len(fileOrder) != len(files) {
		log.Printf("fileOrder length: %d, files length: %d", len(fileOrder), len(files))
		http.Error(w, "upload error: file order does not match number of files", http.StatusBadRequest)
		return
	}

	ordered := services.OrderFiles(files, fileOrder)
	if len(ordered) != len(files) {
		http.Error(w, "upload error: unknown filenames in file order", http.StatusBadRequest)
		return
	}

	fileData, fileNames, err := services.ReadUploadedFiles(ordered)
	if err != nil {
		http.Error(w, "failed to read uploaded files: "+err.Error(), http.StatusInternalServerError)
		return
	}

	tool := utils.TryGetValue(settings, "tool", "")
	var resp *audio.AudioResponse

	switch tool {
	case "compress":
		method := utils.TryGetValue(settings, "method", "")
		switch method {
		case "mb":
			targetSize := utils.TryGetValue(settings, "mb", int32(1))
			resp, err = services.CompressSize(fileData, fileNames, targetSize)
		case "percentage":
			targetPercentage := utils.TryGetValue(settings, "percentage", int32(1))
			resp, err = services.CompressPercentage(fileData, fileNames, targetPercentage)
		case "quality":
			quality := utils.TryGetValue(settings, "quality", "medium")
			resp, err = services.CompressQuality(fileData, fileNames, quality)
		default:
			err = fmt.Errorf("invalid compress method")
		}
	case "convert":
		format := utils.TryGetValue(settings, "format", "mp3")
		bitrate := utils.TryGetValue(settings, "bitrate", int32(128))
		resp, err = services.Convert(fileData, fileNames, format, bitrate)
	case "trim":
		start := utils.TryGetValue(settings, "start", "00:00")
		end := utils.TryGetValue(settings, "end", "59:59")
		mode := utils.TryGetValue(settings, "mode", "keep")
		startSeconds, err1 := utils.ParseToSeconds(start)
		endSeconds, err2 := utils.ParseToSeconds(end)
		if err1 != nil || err2 != nil {
			http.Error(w, "invalid start/end time", http.StatusBadRequest)
			return
		}
		resp, err = services.Trim(fileData[0], fileNames[0], int32(startSeconds), int32(endSeconds), mode)
	case "merge":
		format := utils.TryGetValue(settings, "format", "mp3")
		resp, err = services.Merge(fileData, fileNames, format)
	case "metadata":
		title := utils.TryGetValue(settings, "title", "")
		artist := utils.TryGetValue(settings, "artist", "")
		album := utils.TryGetValue(settings, "album", "")
		year := utils.TryGetValue(settings, "year", "")
		cover, cerr := services.GetCover(r)
		if cerr != nil {
			log.Println("Cover read error:", cerr)
		}
		resp, err = services.Metadata(fileData[0], fileNames[0], title, artist, album, year, cover)
	case "boost":
		mode := utils.TryGetValue(settings, "mode", "boost")
		switch mode {
		case "manual":
			gain := utils.TryGetValue(settings, "gain", int32(0))
			resp, err = services.BoostManual(fileData, fileNames, gain)
		case "normalize":
			resp, err = services.BoostNormalize(fileData, fileNames)
		default:
			err = fmt.Errorf("invalid boost mode")
		}
	default:
		http.Error(w, "invalid tool", http.StatusBadRequest)
		return
	}

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// Download response
	w.Header().Set("Content-Type", "application/octet-stream")
	w.Header().Set("Content-Disposition", fmt.Sprintf(`attachment; filename="%s"`, resp.Filename))
	w.Write(resp.FileData)
}
