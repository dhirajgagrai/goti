FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt install -y build-essential gcc-x86-64-linux-gnu

WORKDIR /goti

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'

CMD ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-gnu"]
