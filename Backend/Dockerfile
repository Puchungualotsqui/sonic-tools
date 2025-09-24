# --- build stage ---
FROM golang:1.24.6-bookworm AS build
WORKDIR /app

# (optional) only if your go.mod pulls private repos or needs git
RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# module deps
COPY go.mod go.sum ./
RUN go mod download

# source
COPY . .

# build
ENV CGO_ENABLED=0
RUN go build -trimpath -ldflags="-s -w" -o /out/backend .

# --- run stage ---
FROM gcr.io/distroless/static-debian12
WORKDIR /app
COPY --from=build /out/backend /app/backend
EXPOSE 4000
ENTRYPOINT ["/app/backend"]
