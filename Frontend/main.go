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
			bodyContent = body.Tool("🎚️ Compress Your Audio", "Compress", settings.Compress())
		case "/convert":
			bodyContent = body.Tool("🔄 Convert Your Audio", "Convert", settings.Convert())
		case "/trim":
			bodyContent = body.Tool("✂️ Trim Your Audio", "Trim", settings.Trim())
		case "/merge":
			bodyContent = body.Tool("➕ Merge Your Audio", "Merge", settings.Merge())
		case "/metadata":
			bodyContent = body.Tool("🏷️ Edit Metadata", "Save metadata", settings.Metadata())
		case "/boost":
			bodyContent = body.Tool("🔊 Volume Booster", "Apply", settings.Boost())
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
