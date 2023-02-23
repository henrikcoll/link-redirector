FROM ekidd/rust-musl-builder AS build

WORKDIR /app
COPY --chown=rust:rust . .
RUN cargo install --path .

FROM scratch

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/link-redirector /link-redirector
CMD ["/link-redirector"]