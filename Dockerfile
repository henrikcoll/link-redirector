FROM rust:alpine AS build

WORKDIR /app
COPY --chown=rust:rust . ./

RUN cargo build --release

FROM alpine

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/link-redirector /link-redirector
CMD ["/link-redirector"]