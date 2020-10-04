FROM rust:1.43.0-slim as build

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src

RUN echo "fn main() {panic!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/rustlabconf*
RUN rm -f target/release/incremental/rustlabconf*
RUN rm -f target/release/deps/rustlabconf*

COPY . .

RUN cargo build --release

# ==================================

FROM gcr.io/distroless/cc

LABEL maintainer="Tommaso Allevi <tomallevi@gmail.com>" \
      name="RustConf Lab" \
      description="Actix Web Example"

COPY --from=build \
      /target/release/rustlabconf \
      /usr/local/bin/

CMD ["rustlabconf"]