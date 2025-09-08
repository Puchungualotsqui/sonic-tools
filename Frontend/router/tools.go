package router

import (
	"fmt"
	"net/http"
	"strings"

	"frontend/components/body"
	"frontend/components/head"
	"frontend/components/settings"

	"github.com/a-h/templ"
)

func GetBodyDynamicTool(r *http.Request) (templ.Component, templ.Component) {
	var bodyContent templ.Component
	var metaData templ.Component = nil

	switch r.URL.Path {
	case "/":
		bodyContent = body.Home()
	case "/compress":
		metaTitle := "Audio Compressor – Free Online Tool"
		metaDesc := "Compress MP3, OGG, AAC, M4A, and more without losing quality. Fast, free, and secure in your browser."

		bodyContent = body.Tool("Online Audio Compressor – Compress MP3, OGG, AAC, M4A, ALAC, WMA",
			"How to Compress Audio Files Online",
			"Upload your audio file and reduce its size without losing quality using our free online compressor.",
			"Compress",
			settings.Compress(),
			".mp3,.ogg,.aac,.m4a,.alac,.wma",
			true)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "/convert":
		metaTitle := "Audio Converter – Free Online Tool"
		metaDesc := "Convert audio files like MP3, WAV, FLAC, OGG, AAC, and more online. Free, secure, and works directly in your browser."

		bodyContent = body.Tool("Audio Converter – Convert MP3, WAV, FLAC, OGG, OPUS, AIFF, AAC, M4A, ALAC, WMA",
			"How to Convert Audio Files Online",
			"Choose your audio file, select the output format, and convert it instantly with our online audio converter.",
			"Convert",
			settings.Convert(""),
			".mp3,.wav,.flac,.ogg,.opus,.aiff,.aac,.m4a,.alac,.wma",
			true)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "/trim":
		metaTitle := "Audio Trimmer – Free Online Tool"
		metaDesc := "Trim MP3, WAV, FLAC, OGG, and other audio files online. Cut silence or unwanted parts quickly and easily."

		bodyContent = body.Tool("Trim Audio Files Online – Cut MP3, WAV, FLAC, OGG, OPUS, AIFF, AAC, M4A, ALAC, WMA",
			"How to Trim Audio Files Online",
			"Upload your track and cut the beginning, end, or any unwanted parts quickly and easily.",
			"Trim",
			settings.Trim(),
			".mp3,.wav,.flac,.ogg,.opus,.aiff,.aac,.m4a,.alac,.wma",
			false)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "/merge":
		metaTitle := "Audio Joiner – Free Online Tool"
		metaDesc := "Merge MP3, WAV, FLAC, OGG, and other audio files online. Combine multiple tracks into one instantly."

		bodyContent = body.Tool("Merge Audio Files Online – Combine MP3, WAV, FLAC, OGG, OPUS, AIFF, AAC, M4A, ALAC, WMA",
			"How to Merge Audio Files Online",
			"Add multiple audio files and combine them into a single track in just a few clicks.",
			"Merge",
			settings.Merge(""),
			".mp3,.wav,.flac,.ogg,.opus,.aiff,.aac,.m4a,.alac,.wma",
			true)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "/metadata":
		metaTitle := "Audio Metadata Editor – Free Online Tool"
		metaDesc := "Edit audio tags like title, artist, and album for MP3, FLAC, OGG, and more. Free, secure, and easy to use."

		bodyContent = body.Tool("Edit Audio Metadata Tags – MP3, WAV, FLAC, OGG, OPUS, AIFF, AAC, M4A, ALAC, WMA",
			"How to Edit Audio Metadata Online",
			"Update song details like title, artist, album, or genre directly in your browser.",
			"Save metadata",
			settings.Metadata(),
			".mp3,.flac,.ogg,.opus,.aac,.m4a,.alac,.wma",
			false)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "/boost":
		metaTitle := "Audio Volume Booster – Free Online Tool"
		metaDesc := "Increase audio volume for MP3, WAV, FLAC, OGG, and more online. Make your audio louder instantly and safely."

		bodyContent = body.Tool("Increase Audio Volume Online – MP3, WAV, FLAC, OGG, OPUS, AIFF, AAC, M4A, ALAC, WMA",
			"How to Boost Audio Volume Online",
			"Make quiet recordings louder without distortion using our free online volume booster.",
			"Apply",
			settings.Boost(),
			".mp3,.wav,.flac,.ogg,.opus,.aiff,.aac,.m4a,.alac,.wma",
			true)
		metaData = head.MetaData(metaTitle, metaDesc)
	default:
		bodyContent = nil
	}
	return bodyContent, metaData
}

func GetBodySpecificTool(r *http.Request) (templ.Component, templ.Component) {
	var bodyContent templ.Component
	var metaData templ.Component

	path := strings.TrimPrefix(r.URL.Path, "/")
	parts := strings.Split(path, "-")
	ext := strings.ToUpper(parts[1])
	fileExt := "." + parts[1]
	switch parts[0] {
	case "convert":
		var thirdPard string = ""
		h1 := fmt.Sprintf("Convert %s Files Online – Free & Fast", ext)
		h2 := fmt.Sprintf("How to Convert %s Files Online", ext)
		desc := fmt.Sprintf("Upload your %s file and convert it instantly to another format in your browser.", ext)
		if len(parts) >= 3 {
			thirdPard = parts[2]
			h1 = fmt.Sprintf("Convert %s to %s Online – Free Audio Converter", ext, strings.ToUpper(parts[2]))
			h2 = fmt.Sprintf("How to Convert %s to %s Online", ext, strings.ToUpper(parts[2]))
			desc = fmt.Sprintf("Upload your %s file and our tool will convert it to %s format quickly and easily.", ext, strings.ToUpper(parts[2]))
		}

		metaTitle := fmt.Sprintf("%s Converter – Free Online Tool", strings.ToUpper(ext))
		if len(parts) >= 3 {
			metaTitle = fmt.Sprintf("%s to %s Converter – Free Online Tool", strings.ToUpper(ext), strings.ToUpper(parts[2]))
		}
		metaDesc := fmt.Sprintf("Convert %s files online in seconds. Free, secure, and works directly in your browser.", strings.ToUpper(ext))
		if len(parts) >= 3 {
			metaDesc = fmt.Sprintf("Convert %s to %s online instantly. 100%% free, safe, and easy to use in your browser.", strings.ToUpper(ext), strings.ToUpper(parts[2]))
		}

		bodyContent = body.Tool(h1, h2, desc, "Convert", settings.Convert(thirdPard), fileExt, true)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "compress":
		h1 := fmt.Sprintf("Compress %s Files Online – Reduce Size Without Losing Quality", ext)
		h2 := fmt.Sprintf("How to Compress %s Files Online", ext)
		desc := fmt.Sprintf("Upload your %s file and reduce its size without losing quality using our online compressor.", ext)

		metaTitle := fmt.Sprintf("%s Compressor – Free Online Tool", strings.ToUpper(ext))
		metaDesc := fmt.Sprintf("Compress %s files online without losing quality. Fast, free, and secure in your browser.", strings.ToUpper(ext))

		bodyContent = body.Tool(h1, h2, desc, "Convert", settings.Compress(), fileExt, true)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "trim":
		h1 := fmt.Sprintf("Trim %s Audio Online – Cut Songs & Remove Silence", ext)
		h2 := fmt.Sprintf("How to Trim %s Files Online", ext)
		desc := fmt.Sprintf("Upload your %s file and cut unwanted parts quickly and easily in your browser.", ext)

		metaTitle := fmt.Sprintf("%s Cutter – Free Online Tool", strings.ToUpper(ext))
		metaDesc := fmt.Sprintf("Trim %s files online to remove silence or unwanted parts. Quick, free, and easy to use.", strings.ToUpper(ext))

		bodyContent = body.Tool(h1, h2, desc, "Trim", settings.Trim(), fileExt, false)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "merge":
		h1 := fmt.Sprintf("Merge %s Files Online – Combine Multiple Tracks into One", ext)
		h2 := fmt.Sprintf("How to Merge %s Files Online", ext)
		desc := fmt.Sprintf("Upload multiple %s files and combine them into a single track instantly.", ext)

		metaTitle := fmt.Sprintf("%s Joiner – Free Online Tool", strings.ToUpper(ext))
		metaDesc := fmt.Sprintf("Merge multiple %s files into one track online. Free, secure, and works in your browser.", strings.ToUpper(ext))

		bodyContent = body.Tool(h1, h2, desc, "Merge", settings.Merge(parts[1]), fileExt, true)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "metadata":
		h1 := fmt.Sprintf("Edit %s Metadata Tags Online – Change Title, Artist, Album", ext)
		h2 := fmt.Sprintf("How to Edit %s Metadata Online", ext)
		desc := fmt.Sprintf("Update the title, artist, album or other tags of your %s file directly in your browser.", ext)

		metaTitle := fmt.Sprintf("Edit %s Metadata – Free Online Tool", strings.ToUpper(ext))
		metaDesc := fmt.Sprintf("Edit %s tags like title, artist, and album online. Quick, free, and secure in your browser.", strings.ToUpper(ext))

		bodyContent = body.Tool(h1, h2, desc, "Save metadata", settings.Metadata(), fileExt, false)
		metaData = head.MetaData(metaTitle, metaDesc)
	case "boost":
		h1 := fmt.Sprintf("Boost %s Volume Online – Make Audio Louder Instantly", ext)
		h2 := fmt.Sprintf("How to Boost %s Volume Online", ext)
		desc := fmt.Sprintf("Upload your %s file and increase its volume without distortion using our free booster.", ext)

		metaTitle := fmt.Sprintf("%s Volume Booster – Free Online Tool", strings.ToUpper(ext))
		metaDesc := fmt.Sprintf("Boost %s volume online and make your audio louder instantly. Free, safe, and easy to use.", strings.ToUpper(ext))

		bodyContent = body.Tool(h1, h2, desc, "Apply", settings.Boost(), fileExt, true)
		metaData = head.MetaData(metaTitle, metaDesc)
	default:
		bodyContent = nil
	}

	return bodyContent, metaData
}
