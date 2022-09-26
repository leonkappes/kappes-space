FROM rust:slim AS build
WORKDIR /build
COPY src src
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release

FROM alpine:3.16 AS release
COPY --from=build /build/target/release/blog-api /bin/blog-api
EXPOSE 80
ENTRYPOINT [ "/bin/blog-api" ]