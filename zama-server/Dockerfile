# Stage 1: Build
FROM golang:latest as builder

WORKDIR /app
COPY . .

RUN go build -o zama-server main.go

CMD ["./zama-server"]
