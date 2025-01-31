FROM rust:alpine AS build

WORKDIR /app/calculator

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=build /app/calculator/target/release/calculator /

CMD ["./calculator"]
