
FROM --platform=$BUILDPLATFORM rust:latest

WORKDIR /usr/src/webgraph-clib-rs

COPY . .

RUN cargo build --release \
    && cp target/release/libwebgraphclibrs.* /usr/local/lib/libwebgraphclibrs.so \
    && cp cbindgen/webgraphclibrs.h /usr/local/include/webgraphclibrs.h