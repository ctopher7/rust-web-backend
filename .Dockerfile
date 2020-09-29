FROM rust:1.46.0 as cargo-build

WORKDIR /usr/src/code

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=cargo-build /usr/src/code/target/release/rust-app /usr/local/bin/rust-app
COPY --from=cargo-build /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1

WORKDIR /usr/src/code
COPY .env.production .env.production

CMD ["rust-app"]