package data

import (
	"encoding/xml"
	"fmt"
	"net/http"
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

var FormatTools = map[string][]string{
	"mp3":  {"convert", "compress", "trim", "merge", "metadata", "boost"},
	"wav":  {"convert", "trim", "merge", "boost"},
	"flac": {"convert", "trim", "merge", "metadata", "boost"},
	"ogg":  {"convert", "compress", "trim", "merge", "metadata", "boost"},
	"opus": {"convert", "compress", "trim", "merge", "metadata", "boost"},
	"aiff": {"convert", "trim", "merge", "boost"},
	"aac":  {"convert", "trim", "merge", "metadata", "boost"},
	"m4a":  {"convert", "trim", "merge", "metadata", "boost"},
	"wma":  {"convert", "trim", "merge", "metadata", "boost"},
}

func SitemapHandler(w http.ResponseWriter, r *http.Request) {
	baseURL := "https://soundtools.dev"
	now := time.Now().Format("2006-01-02")

	var urls []Url

	// 1. Root pages
	rootTools := []string{"compress", "convert", "trim", "merge", "metadata", "boost"}
	urls = append(urls, Url{
		Loc:        baseURL + "/",
		LastMod:    now,
		ChangeFreq: "weekly",
		Priority:   "1.0",
	})
	for _, tool := range rootTools {
		urls = append(urls, Url{
			Loc:        baseURL + "/" + tool,
			LastMod:    now,
			ChangeFreq: "weekly",
			Priority:   "0.9",
		})
	}

	// 2. Tool + Format pages (/tool-format)
	for format, tools := range FormatTools {
		for _, tool := range tools {
			urls = append(urls, Url{
				Loc:        fmt.Sprintf("%s/%s-%s", baseURL, tool, format),
				LastMod:    now,
				ChangeFreq: "weekly",
				Priority:   "0.7",
			})
		}
	}

	// 3. Convert from X → Y pages (/convert-x-y)
	for fromFormat, fromTools := range FormatTools {
		if !contains(fromTools, "convert") {
			continue
		}
		for toFormat := range FormatTools {
			if fromFormat == toFormat {
				continue
			}
			urls = append(urls, Url{
				Loc:        fmt.Sprintf("%s/convert-%s-%s", baseURL, fromFormat, toFormat),
				LastMod:    now,
				ChangeFreq: "weekly",
				Priority:   "0.6",
			})
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

func contains(arr []string, s string) bool {
	for _, a := range arr {
		if a == s {
			return true
		}
	}
	return false
}
