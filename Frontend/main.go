package main

import (
	"log"
	"net/http"

	"frontend/components/body"
	"frontend/components/settings"
	"frontend/views"

	"github.com/a-h/templ"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		var bodyContent templ.Component

		switch r.URL.Path {
		case "/":
			bodyContent = body.Home()
		case "/compress":
			bodyContent = body.Tool("🎚️ Compress Your Audio", "Compress", settings.Compress(), ".mp3,.ogg,.aac", true)
		case "/convert":
			bodyContent = body.Tool("🔄 Convert Your Audio", "Convert", settings.Convert(), ".mp3,.wav,.flac,.ogg,.opus,.aiff", true)
		case "/trim":
			bodyContent = body.Tool("✂️ Trim Your Audio", "Trim", settings.Trim(), ".mp3,.wav,.flac,.ogg,.opus,.aiff", false)
		case "/merge":
			bodyContent = body.Tool("➕ Merge Your Audio", "Merge", settings.Merge(), ".mp3,.wav,.flac,.ogg,.opus,.aiff", true)
		case "/metadata":
			bodyContent = body.Tool("🏷️ Edit Metadata", "Save metadata", settings.Metadata(), ".mp3,.wav,.flac,.ogg,.opus,.aiff", false)
		case "/boost":
			bodyContent = body.Tool("🔊 Volume Booster", "Apply", settings.Boost(), ".mp3,.wav,.flac,.ogg,.opus,.aiff", true)
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

	log.Println("Frontend running at http://localhost:3000")
	log.Fatal(http.ListenAndServe(":3000", nil))

}
