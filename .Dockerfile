FROM rust

WORKDIR /usr/src/code
COPY . .

RUN cargo install --path .

CMD ["rust-app"]