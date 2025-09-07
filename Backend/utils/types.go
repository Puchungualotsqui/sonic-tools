package utils

import (
	"strconv"
	"strings"
)

func GetArrayString(input any) []string {
	raw, ok := input.([]any)
	if !ok {
		return nil
	}

	result := make([]string, 0, len(raw))
	for _, v := range raw {
		if s, ok := v.(string); ok {
			result = append(result, s)
		}
	}
	return result
}

func TryGetValue[T any](m map[string]any, key string, def T) T {
	if v, ok := m[key]; ok {
		if cast, ok := v.(T); ok {
			return cast
		}
	}
	return def
}

func ParseToSeconds(s string) (int, error) {
	if strings.TrimSpace(s) == "" {
		return 0, nil // or return an error if you want it to be invalid
	}

	parts := strings.Split(s, ":")
	total := 0
	multiplier := 1

	// Process from right (seconds) to left (hours)
	for i := len(parts) - 1; i >= 0; i-- {
		val, err := strconv.Atoi(parts[i])
		if err != nil {
			return 0, err
		}
		total += val * multiplier
		multiplier *= 60
	}
	return total, nil
}
