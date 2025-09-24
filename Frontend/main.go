package main

import (
	"log"
	"net/http"

	"frontend/components/head"
	"frontend/static/data"
	"frontend/views"

	"github.com/a-h/templ"

	"frontend/router"
)

const metaTitle = "Sound Tools – Free Online Audio Editor"
const metaDesc = "Edit, convert, compress, trim, merge, and boost your audio files online. 100% free, secure, and works directly in your browser."

func main() {
	http.Handle("/static/",
		http.StripPrefix("/static/",
			http.FileServer(http.Dir("static")),
		),
	)

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		var bodyContent templ.Component
		var metaData templ.Component

		bodyContent, metaData = router.GetBodyDynamicTool(r)
		if bodyContent == nil {
			bodyContent, metaData = router.GetBodySpecificTool(r)
			if bodyContent == nil {
				http.NotFound(w, r)
				return
			}
		}

		if metaData == nil {
			metaData = head.MetaData(metaTitle, metaDesc)

		}

		// Check if request comes from HTMX
		if r.Header.Get("HX-Request") == "true" {
			// Only return the fragment for HTMX swaps
			bodyContent.Render(r.Context(), w)
			metaData.Render(r.Context(), w)
			return
		}

		// Otherwise render full page
		views.Layout(bodyContent, metaData, data.Formats).Render(r.Context(), w)
	})

	http.HandleFunc("/sitemap.xml", data.SitemapHandler)

	// robots.txt
	http.HandleFunc("/robots.txt", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "./static/assets/robots.txt")
	})

	// favicon.ico
	http.HandleFunc("/favicon.ico", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, "./static/assets/favicon.ico")
	})

	log.Println("Frontend running at http://localhost:3000")
	log.Fatal(http.ListenAndServe(":3000", nil))

}
