FROM clux/muslrust:1.46.0 as cargo-build

WORKDIR /usr/src/code

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=cargo-build /usr/src/code/target/x86_64-unknown-linux-musl/release/rust-app /usr/local/bin/rust-app
WORKDIR /usr/src/code
COPY .env.production .env.production

CMD ["rust-app"]