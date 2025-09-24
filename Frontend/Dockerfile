# --- build stage ---
FROM golang:1.24.6-bookworm AS build
WORKDIR /app

RUN apt-get update && apt-get install -y curl git ca-certificates && rm -rf /var/lib/apt/lists/*

# intsall templ
RUN go install github.com/a-h/templ/cmd/templ@latest

# install tailwindcss
RUN curl -fsSL -o /usr/local/bin/tailwindcss \
    https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.13/tailwindcss-linux-x64 \
    && chmod +x /usr/local/bin/tailwindcss

RUN which tailwindcss && tailwindcss --help

# deps
COPY go.mod go.sum ./
RUN go mod download

# app source
COPY . .

# generate + build
RUN templ generate
RUN tailwindcss -i ./static/css/input.css -o ./static/css/output.css --minify

ENV CGO_ENABLED=0
RUN go build -trimpath -ldflags="-s -w" -o /out/frontend .

# --- run stage ---
FROM gcr.io/distroless/static-debian12
WORKDIR /app
COPY --from=build /out/frontend /app/frontend
COPY --from=build /app/static /app/static
EXPOSE 3000
ENTRYPOINT ["/app/frontend"]
