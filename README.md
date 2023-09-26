<div align="center" width="100%">
  <img src="https://github.com/dhirajgagrai/goti/assets/22605432/7c6a01f7-e296-4eb0-8f7b-a3fa210d18c8" width="500" />
</div>

# goti

This was created as part of Project Aegis. It will be included in the Project Aegis repository when WASM compatibility is added.

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

Install [Rust](https://www.rust-lang.org/tools/install) and build using Cargo.
```bash
git clone https://github.com/dhirajgagrai/goti.git
cd goti

# Compile
cargo build --release

# Run
cargo run --release
```
