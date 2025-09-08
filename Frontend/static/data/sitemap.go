package data

import (
	"encoding/xml"
	"fmt"
	"net/http"
	"strings"
	"time"
)

type UrlSet struct {
	XMLName xml.Name `xml:"urlset"`
	Xmlns   string   `xml:"xmlns,attr"`
	Urls    []Url    `xml:"url"`
}

type Url struct {
	Loc        string `xml:"loc"`
	LastMod    string `xml:"lastmod,omitempty"`
	ChangeFreq string `xml:"changefreq,omitempty"`
	Priority   string `xml:"priority,omitempty"`
}

func SitemapHandler(w http.ResponseWriter, r *http.Request) {
	baseURL := "https://yourdomain.com" // TODO: replace with your actual domain
	now := time.Now().Format("2006-01-02")

	var urls []Url

	// Root pages (home + generic tools)
	rootPaths := []string{
		"/", "/compress", "/convert", "/trim", "/merge", "/metadata", "/boost",
	}

	for _, p := range rootPaths {
		urls = append(urls, Url{
			Loc:        fmt.Sprintf("%s%s", baseURL, p),
			LastMod:    now,
			ChangeFreq: "weekly",
			Priority:   "0.9",
		})
	}

	// Dynamic pages from Formats
	for _, f := range Formats {
		for _, t := range f.Tools {
			// base tool path (e.g., /compress-mp3, /merge-flac, /convert-mp3)
			urls = append(urls, Url{
				Loc:        baseURL + t.Path,
				LastMod:    now,
				ChangeFreq: "weekly",
				Priority:   "0.7",
			})

			// special case: Convert also has variants (convert-X-to-Y)
			if t.Name == "Convert" {
				for _, v := range t.Variants {
					urls = append(urls, Url{
						Loc:        fmt.Sprintf("%s%s-%s", baseURL, t.Path, strings.ToLower(v)),
						LastMod:    now,
						ChangeFreq: "weekly",
						Priority:   "0.6",
					})
				}
			}
		}
	}

	// Encode as XML
	w.Header().Set("Content-Type", "application/xml")
	w.WriteHeader(http.StatusOK)

	enc := xml.NewEncoder(w)
	enc.Indent("", "  ")
	enc.Encode(UrlSet{
		Xmlns: "http://www.sitemaps.org/schemas/sitemap/0.9",
		Urls:  urls,
	})
}
