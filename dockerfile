FROM rustlang/rust:nightly-bullseye as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang pkg-config libssl-dev

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY assets ./assets
COPY end2end ./end2end
COPY src ./src
COPY style ./style

# Ensure OpenSSL is statically linked
ENV OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include

RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates libpq-dev \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/chrisbratti-website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000

CMD ["/app/chrisbratti-website"]
