package utils

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
