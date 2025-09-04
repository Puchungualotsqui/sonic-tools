package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/Puchungualotsqui/sonic-tools/components/body"
	"github.com/Puchungualotsqui/sonic-tools/views"
	"github.com/a-h/templ"
)

func uploadHandler(w http.ResponseWriter, r *http.Request) {
	// Parse uploaded file(s)
	r.ParseMultipartForm(10 << 20) // 10MB max just for demo
	files := r.MultipartForm.File["files"]

	for _, fileHeader := range files {
		// Just print an HTML row per file (HTMX will inject this)
		fmt.Fprintf(w, `
		  <div class="p-2 bg-base-200 rounded-lg">
		    <div class="flex justify-between text-sm">
		      <span>%s</span>
		      <span>%.2f MB</span>
		    </div>
		    <progress class="progress progress-primary w-full mt-1" value="100" max="100"></progress>
		  </div>
		`, fileHeader.Filename, float64(fileHeader.Size)/(1024*1024))
	}
}

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		var bodyContent templ.Component

		switch r.URL.Path {
		case "/":
			bodyContent = body.Home()
		case "/compress":
			bodyContent = body.Compress()
		case "/convert":
			bodyContent = body.Convert()
		case "/trim":
			bodyContent = body.Trim()
		case "/merge":
			bodyContent = body.Merge()
		case "/metadata":
			bodyContent = body.Metadata()
		case "/boost":
			bodyContent = body.Boost()
		default:
			http.NotFound(w, r)
			return
		}

		// Check if request comes from HTMX
		if r.Header.Get("HX-Request") == "true" {
			// Only return the fragment for HTMX swaps
			bodyContent.Render(r.Context(), w)
			return
		}

		// Otherwise render full page
		views.Layout(bodyContent).Render(r.Context(), w)
	})

	log.Println("Server running at http://localhost:3000")
	log.Fatal(http.ListenAndServe(":3000", nil))

	http.HandleFunc("/upload", uploadHandler)
	http.ListenAndServe(":8080", nil)
}
