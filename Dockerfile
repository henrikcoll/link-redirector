FROM rust:bullseye AS build

WORKDIR /app
COPY --chown=rust:rust . ./

RUN cargo build --release

FROM debian:bullseye

COPY --from=build /app/target/release/link-redirector /link-redirector
CMD ["/link-redirector"]