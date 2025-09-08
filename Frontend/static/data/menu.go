package data

import "strings"

type Tool struct {
	Name     string
	Path     string
	Variants []string // for conversions
}

type Format struct {
	Name  string
	Tools []Tool
}

var Formats []Format

func init() {
	formats := []string{"MP3", "WAV", "FLAC", "OGG", "OPUS", "AIFF"}

	for _, f := range formats {
		// build tools for each format
		tools := []Tool{
			{
				Name:     "Convert",
				Path:     "/convert-" + lower(f),
				Variants: without(formats, f), // all other formats
			},
			{Name: "Compress", Path: "/compress-" + lower(f)},
			{Name: "Trim", Path: "/trim-" + lower(f)},
			{Name: "Merge", Path: "/merge-" + lower(f)},
			{Name: "Metadata", Path: "/metadata-" + lower(f)},
			{Name: "Boost", Path: "/boost-" + lower(f)},
		}

		Formats = append(Formats, Format{
			Name:  f,
			Tools: tools,
		})
	}
}

func lower(s string) string {
	return strings.ToLower(s)
}

func without(slice []string, skip string) []string {
	res := make([]string, 0, len(slice)-1)
	for _, s := range slice {
		if s != skip {
			res = append(res, s)
		}
	}
	return res
}
