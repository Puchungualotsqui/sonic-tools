package data

import (
	"slices"
	"strings"
)

type Tool struct {
	Name     string
	Path     string
	Variants []string // for conversions
}

type Format struct {
	Name  string
	Tools []Tool
}

// Tool exceptions
var skipTools = map[string][]string{
	"Compress": {"WAV", "FLAC", "AIFF", "ALAC"}, // these can't be compressed
	"Metadata": {"WAV", "AIFF"},
}

// raw list of format names
var FormatNames = []string{"MP3", "WAV", "FLAC", "OGG", "OPUS", "AIFF", "AAC", "M4A", "ALAC", "WMA"}

// exported full format definitions
var Formats []Format

func init() {
	for _, f := range FormatNames {
		tools := []Tool{
			{
				Name:     "Convert",
				Path:     "/convert-" + lower(f),
				Variants: without(FormatNames, f),
			},
		}

		for _, t := range []string{"Compress", "Trim", "Merge", "Metadata", "Boost"} {
			if slices.Contains(skipTools[t], f) {
				continue // skip invalid combos
			}
			tools = append(tools, Tool{
				Name: t,
				Path: "/" + strings.ToLower(t) + "-" + lower(f),
			})
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
