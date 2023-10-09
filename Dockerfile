FROM rustlang/rust:nightly-bullseye as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz

RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz

RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-leptos --version 0.2.0 -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Add GPU support
RUN apt-get update && apt-get install -y libclblast-dev

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release --bin-features clblast -vv

FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/target/release/server /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

RUN apt-get update && apt-get -y upgrade \
    && apt-get install -y \
    ocl-icd-libopencl1 \
    opencl-headers \
    clinfo \
    libclblast-dev

RUN mkdir -p /etc/OpenCL/vendors && \
    echo "libnvidia-opencl.so.1" > /etc/OpenCL/vendors/nvidia.icd

ENV NVIDIA_VISIBLE_DEVICES all
ENV NVIDIA_DRIVER_CAPABILITIES compute,utility
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV MODEL_PATH="/opt/rust-chatbot/open_llama_7b-q5_1-ggjt.bin"
EXPOSE 8080

CMD ["/app/server"]
