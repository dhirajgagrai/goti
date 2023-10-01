FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt install -y g++-mingw-w64-x86-64

WORKDIR /goti

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

CMD ["cargo", "build", "--release", "--target", "x86_64-pc-windows-gnu"]

