# goti

This app was created as a part of Project Aegis. It will be added the the Project Aegis repo when it becomes WASM compatible.

## Usage

### Binaries

Download the binaries from [here](https://github.com/dhirajgagrai/goti/releases).
Give the necessary file permissions to the executable and start it.

Example for the aarch64-appple-darwin release:
```bash
chmod a+x goti_aarch64-apple-darwin
./goti_aarch64-apple-darwin
```

### Build from source

Install [Rust](https://www.rust-lang.org/tools/install) and clone this repo.
```bash
git clone https://github.com/dhirajgagrai/goti.git
cd goti

# Compile
cargo build --release

# Run
cargo run --release
```
