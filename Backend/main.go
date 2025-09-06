package main

import (
	"backend/handlers"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/upload", handlers.UploadHandler)

	log.Println("API running at http://localhost:4000")
	log.Fatal(http.ListenAndServe(":4000", nil))
}
